use std::fmt::Debug;
use std::io::ErrorKind;
use std::net::SocketAddr;
use std::str::{from_utf8, FromStr};
use std::time::Duration;

use contract::{store_measurement, store_measurement_spike};
use log::{debug, error, info, trace, LevelFilter};
use serde::{Deserialize, Serialize};
use subxt::backend::legacy::LegacyRpcMethods;
use subxt::backend::rpc;
use subxt::utils::AccountId32;
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::Keypair;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::{select, sync::watch, time::timeout};

mod contract;
mod emf_contract;
mod storage;

const MAX_MEASUREMENT_VALUE: u128 = 10;

const STORAGE_FILEPATH: &str = "measurements.json";

type Res<T> = Result<T, Error>;

pub(crate) struct Error(String);

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: ToString> From<T> for Error {
    fn from(value: T) -> Self {
        Self(value.to_string())
    }
}

#[derive(Serialize, Deserialize)]
struct RpcRequest {
    value: u128,
}

#[derive(Serialize, Deserialize)]
struct RpcResponse {
    value: u128,
}

#[derive(Clone)]
struct State {
    api: OnlineClient<PolkadotConfig>,
    rpc_legacy: LegacyRpcMethods<PolkadotConfig>,
    keypair: Keypair,
    contract_address: AccountId32,
}

#[tokio::main]
async fn main() -> Res<()> {
    env_logger::builder()
        .filter(None, LevelFilter::Off)
        .filter_module("agent", LevelFilter::Trace)
        .init();
    let (stop_s, stop_r) = watch::channel(());

    let rpc_url = "ws://127.0.0.1:9944";
    let api = OnlineClient::<PolkadotConfig>::from_url(rpc_url).await?;
    let rpc = rpc::RpcClient::from_url(rpc_url).await?;
    let rpc_legacy: LegacyRpcMethods<PolkadotConfig> = LegacyRpcMethods::new(rpc.clone());
    let contract_address: AccountId32 =
        AccountId32::from_str("5Ccs3SPZLqiLKxpfm9TQKLUXXKiEdUMjgVdZPr8NCe4bCSkF")?;
    let keypair = subxt_signer::sr25519::dev::bob();
    let state = State {
        api,
        rpc_legacy,
        keypair,
        contract_address,
    };

    tokio::spawn(async move { start_tcp_server(state, stop_r).await });

    info!("agent started; waiting for termination signal");
    tokio::signal::ctrl_c().await?;
    debug!("received termination signal");
    stop_s.send(())?;
    match timeout(Duration::from_secs(10), stop_s.closed()).await {
        Ok(_) => info!("everything was stopped successfully"),
        Err(e) => {
            error!("failed to stop everything: {}", e)
        }
    }
    Ok(())
}

async fn start_tcp_server(state: State, mut stop_r: watch::Receiver<()>) -> Res<()> {
    let tcp_server_address = "127.0.0.1:3322";
    info!("starting tcp server on {}", tcp_server_address);
    let listener = TcpListener::bind(tcp_server_address).await?;
    loop {
        select! {
            _ = stop_r.changed() => {
                trace!("received stop signal, exit tcp server loop");
                return Ok(());
            }
            connection = listener.accept() => {
                if let Ok(connection) = connection {
                    let state = state.clone();
                    tokio::spawn(async move {
                        if let Err(e) = process_connection(connection, state).await {
                            error!("failed to process connection: {}", e.0)
                        }
                    });
                }
            }
        }
    }
}

async fn process_connection(connection: (TcpStream, SocketAddr), state: State) -> Res<()> {
    let (mut stream, addr) = connection;
    trace!("new rcp client connected: {addr}");

    let mut buf: Vec<u8> = vec![0; 1024];
    let n = stream.read(&mut buf).await;
    let buf = match n {
        Ok(0) => {
            trace!("rpc client disconnected: {}", addr);
            return Ok(());
        }
        Ok(n) => {
            buf.truncate(n);
            // Remove new line if exists.
            if buf.last() == Some(&10) {
                buf.pop();
            }
            buf
        }
        Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
            // This error means that there are no data in socket buffer but it is not closed.
            return Ok(());
        }
        Err(e) => return Err(format!("failed to read from connection: {:?}: {:?}", addr, e).into()),
    };

    trace!("received new data from {} client: {}", addr, from_utf8(&buf)?);
    let req: RpcRequest = serde_json::from_slice(&buf)?;
    handle_rpc_request(&req, state).await?;
    let mut buf: Vec<u8> = serde_json::to_vec(&RpcResponse { value: req.value })?;
    write(&mut stream, &mut buf).await
}

async fn write(stream: &mut TcpStream, buf: &mut Vec<u8>) -> Res<()> {
    // Add new line if not exists.
    if buf.last() != Some(&10) {
        buf.push(10);
    }
    Ok(stream.write_all(buf).await?)
}

async fn handle_rpc_request(req: &RpcRequest, state: State) -> Res<()> {
    if req.value > MAX_MEASUREMENT_VALUE {
        return store_measurement_spike(
            &state.api,
            &state.rpc_legacy,
            &state.keypair,
            state.contract_address,
            req.value,
        )
        .await;
    }
    let last_iteration = storage::save(STORAGE_FILEPATH, req.value)?;
    if !last_iteration.is_empty() {
        let mut avg_value = 0;
        for value in &last_iteration {
            avg_value += value;
        }
        avg_value /= last_iteration.len() as u128;
        store_measurement(
            &state.api,
            &state.rpc_legacy,
            &state.keypair,
            state.contract_address,
            avg_value,
        )
        .await?;
    }
    Ok(())
}
