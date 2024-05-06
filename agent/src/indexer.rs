use std::{
    collections::{BTreeMap, HashMap},
    fs::OpenOptions,
    io::ErrorKind,
    str::FromStr,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Extension, Json, Router,
};
use contract_transcode::ContractMessageTranscoder;
use log::{debug, error, info, trace};
use serde::{Deserialize, Serialize};
use sqlx::{Connection, SqliteConnection};
use subxt::{
    backend::rpc::RpcClient,
    events::{EventDetails, Events, StaticEvent},
    ext::sp_core::{bytes::to_hex, H256},
    rpc_params,
    utils::AccountId32,
    OnlineClient, PolkadotConfig,
};
use tokio::{
    sync::{mpsc, Mutex},
    time::sleep,
};

use crate::{
    emf_contract::api::{
        contracts::events::ContractEmitted,
        runtime_types::pallet_contracts::pallet::Event as ContractsEvent,
    },
    Error,
};
use crate::{
    emf_contract::{self, api::runtime_types::contracts_node_runtime::RuntimeEvent},
    Res,
};

type Task = (u64, Res<(SystemTime, Option<Events<PolkadotConfig>>)>);

#[derive(Debug, scale::Decode)]
struct EntityCreated {
    entity: AccountId32,
}

impl DatabaseSaver for EntityCreated {
    async fn save(self, db: &DatabasePointer, timestamp: SystemTime) -> Res<()> {
        sqlx::query("insert into entities (account_id, created_at) values (?1, ?2)")
            .bind(self.entity.to_string())
            .bind(timestamp.duration_since(UNIX_EPOCH)?.as_secs() as u32)
            .execute(&mut db.lock().await.conn)
            .await?;
        Ok(())
    }
}

#[derive(scale::Decode)]
struct SubEntityCreated {
    entity: AccountId32,
    sub_entity: AccountId32,
    location: String,
}

impl DatabaseSaver for SubEntityCreated {
    async fn save(self, db: &DatabasePointer, timestamp: SystemTime) -> Res<()> {
        sqlx::query("insert into sub_entities (entity, account_id, location, created_at) values (?1, ?2, ?3, ?4)")
            .bind(self.entity.to_string())
            .bind(self.sub_entity.to_string())
            .bind(self.location)
            .bind(timestamp.duration_since(UNIX_EPOCH)?.as_secs() as u32)
            .execute(&mut db.lock().await.conn)
            .await?;
        Ok(())
    }
}

#[derive(scale::Decode)]
struct NewSpike {
    _entity: AccountId32,
    sub_entity: AccountId32,
    value: u128,
}

impl DatabaseSaver for NewSpike {
    async fn save(self, db: &DatabasePointer, timestamp: SystemTime) -> Res<()> {
        sqlx::query("insert into spikes (sub_entity, value, created_at) values (?1, ?2, ?3)")
            .bind(self.sub_entity.to_string())
            .bind(self.value as u32)
            .bind(timestamp.duration_since(UNIX_EPOCH)?.as_secs() as u32)
            .execute(&mut db.lock().await.conn)
            .await?;
        Ok(())
    }
}

#[derive(scale::Decode)]
struct TooManySpikes {
    _entity: AccountId32,
    sub_entity: AccountId32,
}

impl DatabaseSaver for TooManySpikes {
    async fn save(self, db: &DatabasePointer, timestamp: SystemTime) -> Res<()> {
        sqlx::query("insert into too_many_spikes (sub_entity, created_at) values (?1, ?2)")
            .bind(self.sub_entity.to_string())
            .bind(timestamp.duration_since(UNIX_EPOCH)?.as_secs() as u32)
            .execute(&mut db.lock().await.conn)
            .await?;
        Ok(())
    }
}

#[derive(scale::Decode)]
struct CertificateReady {
    _entity: AccountId32,
    sub_entity: AccountId32,
}

impl DatabaseSaver for CertificateReady {
    async fn save(self, db: &DatabasePointer, timestamp: SystemTime) -> Res<()> {
        sqlx::query("insert into ready_certificates (sub_entity, created_at) values (?1, ?2)")
            .bind(self.sub_entity.to_string())
            .bind(timestamp.duration_since(UNIX_EPOCH)?.as_secs() as u32)
            .execute(&mut db.lock().await.conn)
            .await?;
        Ok(())
    }
}

trait DatabaseSaver {
    async fn save(self, db: &DatabasePointer, timestamp: SystemTime) -> Res<()>;
}

pub(crate) async fn run(api: OnlineClient<PolkadotConfig>, rpc: RpcClient) -> Res<()> {
    let database = Arc::new(Mutex::new(Database::new().await?));
    let database_ = database.clone();
    tokio::spawn(async move { run_indexer(api, rpc, database_).await });
    tokio::spawn(async move {
        if let Err(e) = run_api(database).await {
            error!("failed to run api: {:?}", e)
        }
    });
    Ok(())
}

async fn run_indexer(api: OnlineClient<PolkadotConfig>, rpc: RpcClient, database: DatabasePointer) {
    let topics = match init_topics() {
        Ok(topics) => topics,
        Err(e) => {
            error!("failed to init topics: {:?}", e);
            return;
        }
    };

    let mut current_block_index: u64 = 0;
    let mut workers: usize = 2;
    loop {
        let saved_current_block_index = current_block_index;
        let saved_workers = workers;
        debug!("current block to sync is {current_block_index}; workers = {workers}");
        match process(&mut current_block_index, workers, &api, &rpc, &topics, &database).await {
            Ok(no_more_events) => {
                if no_more_events {
                    current_block_index = saved_current_block_index;
                    workers /= 2;
                    if workers == 0 {
                        workers = 1;
                    }
                    trace!("indexer synced all blocks; waiting for new; current workers count is {workers}");
                    sleep(Duration::from_secs(2)).await;
                }
            }
            Err(e) => {
                error!("failed to process starting from {saved_current_block_index}: {:?}", e);
                current_block_index = saved_current_block_index;
                workers = saved_workers;
                sleep(Duration::from_secs(10)).await;
            }
        }
    }
}

fn init_topics() -> Res<HashMap<String, String>> {
    let transcoder = ContractMessageTranscoder::load("assets/emf_contract.metadata.json")?;
    // Topic hash to it's name.
    let mut topics: HashMap<String, String> = HashMap::new();
    for event_meta in transcoder.metadata().spec().events() {
        let topic = to_hex(
            event_meta
                .signature_topic()
                .ok_or::<Error>("failed to find topic signature".into())?
                .as_bytes(),
            false,
        );
        topics.insert(topic, event_meta.label().clone());
    }
    Ok(topics)
}

async fn process(
    current_block_index: &mut u64,
    workers: usize,
    api: &OnlineClient<PolkadotConfig>,
    rpc: &RpcClient,
    topics: &HashMap<String, String>,
    database: &DatabasePointer,
) -> Res<bool> {
    let (res_s, res_r) = mpsc::channel::<Task>(1);

    start_workers(current_block_index, workers, &res_s, api, rpc);
    let results = wait_results(workers, res_r).await?;

    for res in results {
        let (timestamp, res) = res.1;
        let events = match res {
            Some(events) => events,
            // It means there are no events in the block.
            // Usually it means there is no block with such index
            // and we need to wait for it.
            None => return Ok(true),
        };
        for event in events.iter().flatten() {
            process_event(event, timestamp, topics, database).await?;
        }
    }

    Ok(false)
}

async fn process_event(
    event: EventDetails<PolkadotConfig>,
    timestamp: SystemTime,
    topics: &HashMap<String, String>,
    database: &DatabasePointer,
) -> Res<()> {
    if event.variant_name() != ContractEmitted::EVENT {
        return Ok(());
    }
    // Usually first topic is our actual smart contract (EMF) event.
    let topic = event.topics()[0];
    let event = event.as_root_event::<RuntimeEvent>()?;
    if let RuntimeEvent::Contracts(ContractsEvent::ContractEmitted { data, .. }) = event {
        let topic_name = topics
            .get(&to_hex(&topic.0, false))
            .ok_or::<Error>("failed to find topic".into())?
            .as_str();
        match topic_name {
            "EntityCreated" => {
                prepare_event_data::<EntityCreated>(data, database, timestamp).await?;
            }
            "SubEntityCreated" => {
                prepare_event_data::<SubEntityCreated>(data, database, timestamp).await?;
            }
            "NewSpike" => prepare_event_data::<NewSpike>(data, database, timestamp).await?,
            "TooManySpikes" => {
                prepare_event_data::<TooManySpikes>(data, database, timestamp).await?
            }
            "CertificateReady" => {
                prepare_event_data::<CertificateReady>(data, database, timestamp).await?
            }
            _ => return Ok(()),
        }
    }
    Ok(())
}

async fn prepare_event_data<T>(
    data: Vec<u8>,
    database: &DatabasePointer,
    timestamp: SystemTime,
) -> Res<()>
where
    T: scale::Decode + DatabaseSaver,
{
    let data: T = scale::decode_from_bytes(data.into())?;
    data.save(database, timestamp).await?;
    Ok(())
}

fn start_workers(
    current_block_index: &mut u64,
    workers: usize,
    res_s: &mpsc::Sender<Task>,
    api: &OnlineClient<PolkadotConfig>,
    rpc: &RpcClient,
) {
    for _ in 0..workers {
        *current_block_index += 1;
        let local_block_index = *current_block_index;
        let api = api.clone();
        let rpc = rpc.clone();
        let res_s_ = res_s.clone();
        tokio::spawn(async move {
            let res = get_events(local_block_index, &api, &rpc).await;
            if let Err(e) = res_s_.send((local_block_index, res)).await {
                error!("failed to send ok result by {local_block_index} to the channel: {e}");
            }
        });
    }
}

async fn get_events(
    block_index: u64,
    api: &OnlineClient<PolkadotConfig>,
    rpc: &RpcClient,
) -> Res<(SystemTime, Option<Events<PolkadotConfig>>)> {
    trace!("get events in {} block", block_index);
    let res: Result<H256, subxt::Error> =
        rpc.request("chain_getBlockHash", rpc_params![block_index]).await;
    let hash = match res {
        Ok(hash) => hash,
        Err(subxt::Error::Serialization(_)) => return Ok((SystemTime::UNIX_EPOCH, None)),
        Err(e) => return Err(e.into()),
    };
    let block = api.blocks().at(hash).await?;

    let timestamp = block
        .extrinsics()
        .await?
        .find_first::<emf_contract::api::timestamp::calls::types::Set>()?
        .ok_or::<Error>("".into())?
        .value
        .now;
    let timestamp = UNIX_EPOCH + Duration::from_secs(timestamp);

    let events = block.events().await?;
    Ok((timestamp, Some(events)))
}

async fn wait_results(
    workers: usize,
    mut res_r: mpsc::Receiver<Task>,
) -> Res<BTreeMap<u64, (SystemTime, Option<Events<PolkadotConfig>>)>> {
    let mut results: BTreeMap<u64, (SystemTime, Option<Events<PolkadotConfig>>)> = BTreeMap::new();
    let mut last_err: Option<Error> = None;
    for _ in 0..workers {
        let (block, res) = match res_r.recv().await {
            Some(res) => res,
            None => return Err("failed to receive a result from the thread, it is none".into()),
        };
        match res {
            Ok(res) => {
                results.insert(block, (res.0, res.1));
            }
            Err(e) => last_err = Some(e),
        }
    }
    match last_err {
        Some(e) => Err(e),
        None => Ok(results),
    }
}

#[derive(sqlx::FromRow, Serialize)]
struct Entity {
    account_id: String,
    created_at: u32,
}

#[derive(sqlx::FromRow, Serialize)]
struct SubEntity {
    entity: String,
    account_id: String,
    location: String,
    created_at: u32,
}

#[derive(sqlx::FromRow, Serialize)]
struct Spike {
    sub_entity: String,
    value: String,
    created_at: u32,
}

#[derive(sqlx::FromRow, Serialize)]
struct TooManySpike {
    sub_entity: String,
    created_at: u32,
}

#[derive(sqlx::FromRow, Serialize)]
struct ReadyCertificate {
    sub_entity: String,
    created_at: u32,
}

type DatabasePointer = Arc<Mutex<Database>>;

struct Database {
    conn: SqliteConnection,
}

impl Database {
    async fn new() -> Res<Self> {
        const DSN: &str = "sqlite:emf.indexer.sqlite";

        // Create file if not exists to be able to open and migrate.
        let file_name = DSN.split(':').collect::<Vec<&str>>()[1];
        if let Err(e) =
            OpenOptions::new().read(true).create(true).append(true).create_new(true).open(file_name)
        {
            match e.kind() {
                ErrorKind::AlreadyExists => (),
                _ => return Err(e.into()),
            }
        }

        let mut conn = SqliteConnection::connect(DSN).await?;
        conn.ping().await?;

        let migrator = sqlx::migrate!("./migrations/");
        migrator.run_direct(&mut conn).await?;

        Ok(Self { conn })
    }

    async fn read_entities(&mut self) -> Res<Vec<Entity>> {
        let entities: Vec<Entity> =
            sqlx::query_as::<_, Entity>("select * from entities").fetch_all(&mut self.conn).await?;
        Ok(entities)
    }

    async fn read_sub_entities(&mut self, entity: AccountId32) -> Res<Vec<SubEntity>> {
        let sub_entities: Vec<SubEntity> =
            sqlx::query_as::<_, SubEntity>("select * from sub_entities where entity = ?1")
                .bind(entity.to_string())
                .fetch_all(&mut self.conn)
                .await?;
        Ok(sub_entities)
    }

    async fn read_spikes(&mut self, sub_entity: AccountId32) -> Res<Vec<Spike>> {
        let spikes: Vec<Spike> =
            sqlx::query_as::<_, Spike>("select * from spikes where sub_entity = ?1")
                .bind(sub_entity.to_string())
                .fetch_all(&mut self.conn)
                .await?;
        Ok(spikes)
    }

    async fn read_too_many_spikes(&mut self, sub_entity: AccountId32) -> Res<Vec<TooManySpike>> {
        let too_many_spikes: Vec<TooManySpike> = sqlx::query_as::<_, TooManySpike>(
            "select * from too_many_spikes where sub_entity = ?1",
        )
        .bind(sub_entity.to_string())
        .fetch_all(&mut self.conn)
        .await?;
        Ok(too_many_spikes)
    }

    async fn read_ready_certificates(
        &mut self,
        sub_entity: AccountId32,
    ) -> Res<Vec<ReadyCertificate>> {
        let ready_certificates: Vec<ReadyCertificate> = sqlx::query_as::<_, ReadyCertificate>(
            "select * from ready_certificates where sub_entity = ?1",
        )
        .bind(sub_entity.to_string())
        .fetch_all(&mut self.conn)
        .await?;
        Ok(ready_certificates)
    }
}

struct ErrorResponse {
    status_code: StatusCode,
    message: String,
}

impl From<Error> for ErrorResponse {
    fn from(value: Error) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: value.0,
        }
    }
}

impl<T: ToString> From<T> for ErrorResponse {
    fn from(value: T) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: value.to_string(),
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        if self.status_code == StatusCode::INTERNAL_SERVER_ERROR {
            error!("internal server error: {}", self.message);
        }
        if self.message.is_empty() {
            self.status_code.into_response()
        } else {
            (self.status_code, self.message).into_response()
        }
    }
}

struct QueryArray<T>(T);

#[axum::async_trait]
impl<S, T> FromRequestParts<S> for QueryArray<T>
where
    S: Send + Sync,
    T: serde::de::DeserializeOwned + Default,
{
    type Rejection = ErrorResponse;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let query = match parts.uri.query() {
            Some(query) => query,
            None => return Ok(Self(T::default())),
        };
        let data = match serde_qs::from_str::<T>(query) {
            Ok(data) => data,
            Err(e) => return Err(format!("failed to decode query params: {e}").into()),
        };
        Ok(Self(data))
    }
}

async fn run_api(database: DatabasePointer) -> Res<()> {
    let app = Router::new()
        .route("/entities", get(get_entities))
        .route("/sub-entities", get(get_sub_entities))
        .route("/spikes", get(get_spikes))
        .route("/too-many-spikes", get(get_too_many_spikes))
        .route("/ready-certificates", get(get_ready_certificates))
        .layer(Extension(database))
        .fallback(fallback);
    let addr = "127.0.0.1:9494";
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("listen on {addr} for HTTP requests");
    axum::serve(listener, app).await?;
    Ok(())
}

#[derive(Deserialize, Default)]
struct QueryParams {
    account_id: String,
}

async fn get_entities(
    Extension(database): Extension<DatabasePointer>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let entities = database.lock().await.read_entities().await?;
    Ok((StatusCode::OK, Json(entities)))
}

async fn get_sub_entities(
    Extension(database): Extension<DatabasePointer>,
    QueryArray(params): QueryArray<QueryParams>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let entity: AccountId32 = AccountId32::from_str(&params.account_id)?;
    let sub_entities = database.lock().await.read_sub_entities(entity).await?;
    Ok((StatusCode::OK, Json(sub_entities)))
}

async fn get_spikes(
    Extension(database): Extension<DatabasePointer>,
    QueryArray(params): QueryArray<QueryParams>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let sub_entity: AccountId32 = AccountId32::from_str(&params.account_id)?;
    let spikes = database.lock().await.read_spikes(sub_entity).await?;
    Ok((StatusCode::OK, Json(spikes)))
}

async fn get_too_many_spikes(
    Extension(database): Extension<DatabasePointer>,
    QueryArray(params): QueryArray<QueryParams>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let sub_entity: AccountId32 = AccountId32::from_str(&params.account_id)?;
    let too_many_spikes = database.lock().await.read_too_many_spikes(sub_entity).await?;
    Ok((StatusCode::OK, Json(too_many_spikes)))
}

async fn get_ready_certificates(
    Extension(database): Extension<DatabasePointer>,
    QueryArray(params): QueryArray<QueryParams>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let sub_entity: AccountId32 = AccountId32::from_str(&params.account_id)?;
    let ready_certificates = database.lock().await.read_ready_certificates(sub_entity).await?;
    Ok((StatusCode::OK, Json(ready_certificates)))
}

async fn fallback() -> impl IntoResponse {
    StatusCode::NOT_FOUND
}
