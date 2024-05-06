use std::{
    collections::{BTreeMap, HashMap},
    fs::OpenOptions,
    io::{Bytes, ErrorKind},
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
use contract_transcode::{ContractMessageTranscoder, Transcoder};
use log::{debug, error, info, trace};
use scale::Decode;
use serde::{Deserialize, Serialize};
use sqlx::{Connection, QueryBuilder, SqliteConnection};
use subxt::{
    backend::rpc::RpcClient,
    events::{EventDetails, Events, StaticEvent},
    ext::sp_core::{bytes::to_hex, hexdisplay::AsBytesRef, H256},
    rpc_params,
    utils::AccountId32,
    OnlineClient, PolkadotConfig,
};
use tokio::{
    sync::{mpsc, Mutex},
    time::sleep,
};

use crate::{emf_contract::api::runtime_types::contracts_node_runtime::RuntimeEvent, Res};
use crate::{
    emf_contract::api::{
        contracts::events::ContractEmitted,
        runtime_types::pallet_contracts::pallet::Event as ContractsEvent,
    },
    Error,
};

type Task = (u64, Res<Option<Events<PolkadotConfig>>>);

#[derive(Debug, Decode)]
struct EntityCreated {
    entity: AccountId32,
}

#[derive(Debug, Decode)]
struct SubEntityCreated {
    entity: AccountId32,
    sub_entity: AccountId32,
    location: String,
}

#[derive(Debug, Decode)]
struct NewSpike {
    entity: AccountId32,
    sub_entity: AccountId32,
    value: u128,
}

#[derive(Debug, Decode)]
struct TooMuchSpikes {
    entity: AccountId32,
    sub_entity: AccountId32,
}

#[derive(Debug, Decode)]
struct CertificateReady {
    entity: AccountId32,
    sub_entity: AccountId32,
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
        match process(&mut current_block_index, workers, &api, &rpc, &topics).await {
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
                .ok_or::<Error>("failed to find topic signature".to_string().into())?
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
) -> Res<bool> {
    let (res_s, res_r) = mpsc::channel::<Task>(1);

    start_workers(current_block_index, workers, &res_s, api, rpc);
    let results = wait_results(workers, res_r).await?;

    for res in results {
        let events = match res.1 {
            Some(events) => events,
            // It means there are no events in the block.
            // Usually it means there is no block with such index
            // and we need to wait for it.
            None => return Ok(true),
        };
        for event in events.iter().flatten() {
            process_event(event, topics).await?;
        }
    }

    Ok(false)
}

async fn process_event(
    event: EventDetails<PolkadotConfig>,
    topics: &HashMap<String, String>,
) -> Res<()> {
    if event.variant_name() != ContractEmitted::EVENT {
        return Ok(());
    }
    // Usually first topic is our actual smart contract (EMF) event.
    let topic = event.topics()[0];
    let event = event.as_root_event::<RuntimeEvent>().unwrap(); // todo: delete unwrap
    if let RuntimeEvent::Contracts(ContractsEvent::ContractEmitted { mut data, .. }) = event {
        let topic_name = topics.get(&to_hex(&topic.0, false)).unwrap().as_str(); // todo: delete unwrap
        eprintln!("NEW EVENT: {}", topic_name);
        match topic_name {
            "EntityCreated" => {
                let data = decode_event_data::<EntityCreated>(data);
                eprintln!("{:?}", data)
            }
            "SubEntityCreated" => {
                let data = decode_event_data::<SubEntityCreated>(data);
                eprintln!("{:?}", data)
            }
            "CertificateReady" => {
                let data = decode_event_data::<CertificateReady>(data);
                eprintln!("{:?}", data)
            }
            "NewSpike" => {
                let data = decode_event_data::<NewSpike>(data);
                eprintln!("{:?}", data)
            }
            "TooMuchSpikes" => {
                let data = decode_event_data::<TooMuchSpikes>(data);
                eprintln!("{:?}", data)
            }
            _ => return Ok(()),
        }
    }
    Ok(())
}

fn decode_event_data<T>(data: Vec<u8>) -> Res<T>
where
    T: scale::Decode,
{
    Ok(scale::decode_from_bytes(data.into())?)
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
) -> Res<Option<Events<PolkadotConfig>>> {
    trace!("get events in {} block", block_index);
    let res: Result<H256, subxt::Error> =
        rpc.request("chain_getBlockHash", rpc_params![block_index]).await;
    let hash = match res {
        Ok(hash) => hash,
        Err(subxt::Error::Serialization(_)) => return Ok(None),
        Err(e) => return Err(e.into()),
    };
    let events = api.blocks().at(hash).await?.events().await?;
    Ok(Some(events))
}

async fn wait_results(
    workers: usize,
    mut res_r: mpsc::Receiver<Task>,
) -> Res<BTreeMap<u64, Option<Events<PolkadotConfig>>>> {
    let mut results: BTreeMap<u64, Option<Events<PolkadotConfig>>> = BTreeMap::new();
    let mut last_err: Option<Error> = None;
    for _ in 0..workers {
        let (block, res) = match res_r.recv().await {
            Some(res) => res,
            None => return Err("failed to receive a result from the thread, it is none".into()),
        };
        match res {
            Ok(res) => {
                results.insert(block, res);
            }
            Err(e) => last_err = Some(e),
        }
    }
    match last_err {
        Some(e) => Err(e),
        None => Ok(results),
    }
}

#[derive(sqlx::FromRow)]
struct DatabaseDevice {
    address: String,
    version: String,
    data: Vec<u8>,
    updated_at: i64,
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

    // async fn save(&mut self) -> Res<()> {
    //     sqlx::query(
    //         r#"
    //         "#,
    //     )
    //     .execute(&mut self.conn)
    //     .await?;
    //     Ok(())
    // }

    // async fn query(&mut self, params: GetDevicesParams) -> Res<Vec<DatabaseDevice>> {
    //     let mut query: QueryBuilder<sqlx::Sqlite> = Self::prepare_query::<sqlx::Sqlite>(&params)?;
    //     trace!("sql query: {}", query.sql());
    //     let query = query.build_query_as::<DatabaseDevice>();
    //     let devices = query.fetch_all(&mut self.conn).await?;
    //     Ok(devices)
    // }

    // // todo: fix it
    // fn prepare_query<'a, DB: sqlx::Database>(
    //     params: &'a GetDevicesParams,
    // ) -> Res<QueryBuilder<'a, DB>>
    // where
    //     std::string::String: sqlx::Encode<'a, DB>,
    //     std::string::String: sqlx::Type<DB>,
    //     u32: sqlx::Encode<'a, DB>,
    //     u32: sqlx::Type<DB>,
    //     f64: sqlx::Encode<'a, DB>,
    //     f64: sqlx::Type<DB>,
    // {
    //     let mut query: QueryBuilder<DB> = QueryBuilder::new("select * from devices");
    //     if let Some(address) = &params.address {
    //         query.push(" where address = ");
    //         query.push_bind(address);
    //         return Ok(query);
    //     }
    //     let filters_len = params.filters.len();
    //     if filters_len != 0 {
    //         query.push(" where ");
    //         for (i, filter) in params.filters.iter().enumerate() {
    //             Self::is_filter_allowed(filter)?;
    //             if i != 0 {
    //                 query.push(" AND ");
    //             }
    //             query.push(format!(
    //                 "json_extract(data, '$.{}') {} ",
    //                 filter.field, filter.condition
    //             ));
    //             push_bind(&mut query, &filter.value);
    //         }
    //     }
    //     query.push(" order by updated_at desc");
    //     query.push(" limit ").push_bind(params.limit).push(" offset ").push_bind(params.offset);
    //     Ok(query)
    // }

    // I didn't find a way to properly bind JSON field name and condition to sql query,
    // so it is required to manually check for allowed fields and conditions.
    // For value we don't need this check as we can bind it.
    fn is_filter_allowed(filter: &Filter) -> Res<()> {
        Self::is_field_allowed(&filter.field)?;
        Self::is_condition_allowed(&filter.condition)?;
        Ok(())
    }

    // todo: fix it
    fn is_field_allowed(field: &str) -> Res<()> {
        if matches!(field, "data_type" | "location" | "price_access" | "price_pin") {
            return Ok(());
        }
        Err("received untrusted filter".into())
    }

    fn is_condition_allowed(field: &str) -> Res<()> {
        if matches!(field, "=" | "<" | ">") {
            return Ok(());
        }
        Err("received untrusted condition".into())
    }
}

fn push_bind<'a, DB: sqlx::Database>(query: &mut QueryBuilder<'a, DB>, value: &'a Value)
where
    std::string::String: sqlx::Encode<'a, DB>,
    std::string::String: sqlx::Type<DB>,
    f64: sqlx::Encode<'a, DB>,
    f64: sqlx::Type<DB>,
{
    match value {
        Value::String(string) => query.push_bind(string),
        Value::F64(f64) => query.push_bind(f64),
    };
}

struct ErrorResponse {
    status_code: StatusCode,
    message: String,
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

async fn run_api(database: DatabasePointer) -> Res<()> {
    let app = Router::new()
        // .route("/devices", get(get_devices))
        .layer(Extension(database))
        .fallback(fallback);
    let addr = "127.0.0.1:9494";
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("listen on {addr} for HTTP requests");
    axum::serve(listener, app).await?;
    Ok(())
}

struct QueryArray<T>(pub T);

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

// // todo: fix it
// #[derive(Deserialize)]
// struct GetDevicesParams {
//     address: Option<String>,
//     #[serde(default)]
//     filters: Vec<Filter>,
//     #[serde(default)]
//     limit: u32,
//     #[serde(default)]
//     offset: u32,
// }

// impl Default for GetDevicesParams {
//     fn default() -> Self {
//         Self {
//             address: None,
//             filters: vec![],
//             limit: 10,
//             offset: 0,
//         }
//     }
// }

#[derive(Deserialize)]
struct Filter {
    field: String,
    condition: String, // "=", "<", ">"
    value: Value,
}

enum Value {
    String(String),
    F64(f64),
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.parse::<f64>() {
            Ok(v) => Ok(Value::F64(v)),
            _ => Ok(Value::String(value)),
        }
    }
}

// #[derive(Serialize, Deserialize)]
// struct DeviceResponse {
//     address: String,
//     version: String,
//     device: serde_json::Value,
//     updated_at: u64,
// }

// async fn get_devices(
//     Extension(database): Extension<DatabasePointer>,
//     QueryArray(params): QueryArray<GetDevicesParams>,
// ) -> Result<impl IntoResponse, ErrorResponse> {
//     for filter in &params.filters {
//         if Database::is_filter_allowed(filter).is_err() {
//             return Err(format!("{} field is not supporting for filtering", filter.field).into());
//         }
//     }
//     let internal_devices = database.lock().await.query(params).await?;
//     let mut external_devices: Vec<DeviceResponse> = Vec::with_capacity(internal_devices.len());
//     for internal_device in &internal_devices {
//         let device: serde_json::Value = {
//             match internal_device.version.as_str() {
//                 V1 => {
//                     let device: serde_json::Value = serde_json::from_slice(&internal_device.data)?;
//                     device
//                 }
//                 _ => {
//                     return Err(format!(
//                         "unknown version to convert internal device to external :{}",
//                         internal_device.version
//                     )
//                     .into())
//                 }
//             }
//         };
//         external_devices.push(DeviceResponse {
//             address: internal_device.address.clone(),
//             version: internal_device.version.clone(),
//             device,
//             updated_at: internal_device.updated_at as u64,
//         })
//     }
//     Ok((StatusCode::OK, Json(external_devices)))
// }

async fn fallback() -> impl IntoResponse {
    StatusCode::NOT_FOUND
}
