use std::{collections::HashMap, process::Stdio, str::from_utf8, time::Duration};

use contract_transcode::ContractMessageTranscoder;
use emf_contract::api::{
    self,
    runtime_types::{
        contracts_node_runtime::RuntimeEvent, pallet_contracts::pallet::Event as ContractsEvent,
    },
};
use serde::{Deserialize, Serialize};
use subxt::{
    events::{Events, StaticEvent},
    ext::sp_core::bytes::to_hex,
    OnlineClient, PolkadotConfig,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::oneshot,
    time::{sleep, timeout},
};

use crate::emf_contract::api::contracts::events::ContractEmitted;

mod emf_contract;

#[derive(Serialize, Deserialize)]
struct RpcRequest {
    value: u128,
}

#[derive(Serialize, Deserialize)]
struct RpcResponse {
    value: u128,
}

struct ChildProcess {
    child: tokio::process::Child,
}

fn start_substrate_contracts_node() -> ChildProcess {
    const NODE_DATA_PATH: &str = "data-substrate";
    std::fs::remove_dir_all(NODE_DATA_PATH).unwrap();
    let child = tokio::process::Command::new("substrate-contracts-node")
        .args(vec![
            "--no-telemetry",
            "--dev",
            "-d",
            NODE_DATA_PATH,
            "-l",
            "info",
            "--unsafe-rpc-external",
            "--rpc-external",
            "--rpc-methods",
            "unsafe",
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .unwrap();
    ChildProcess { child }
}

async fn deploy_smart_contract() -> String {
    tokio::process::Command::new("cargo")
        .args(vec!["contract", "upload", "--suri", "//Alice", "-x"])
        .kill_on_drop(true)
        .current_dir("../emf_contract")
        .output()
        .await
        .unwrap();

    let output = tokio::process::Command::new("cargo")
        .args(vec![
            "contract",
            "instantiate",
            "--suri",
            "//Alice",
            "--args",
            "10",
            "--args",
            "2",
            "--args",
            "1",
            "--args",
            "1",
            "--args",
            "360",
            "--args",
            "2",
            "-x",
            "--skip-confirm",
        ])
        .kill_on_drop(true)
        .current_dir("../emf_contract")
        .output()
        .await
        .unwrap();

    if output.stdout.is_empty() {
        unreachable!()
    }
    let lines: Vec<&[u8]> = output.stdout.split(|b| *b == 10).collect();
    for line in lines {
        let line = from_utf8(line).unwrap();
        if line.contains("Contract ") {
            let split: Vec<&str> = line.split(' ').collect();
            if split.len() != 6 {
                continue;
            }
            return split[5].to_string();
        }
    }
    unreachable!()
}

fn start_agent(smart_contract_address: &str) -> ChildProcess {
    let child = tokio::process::Command::new("target/debug/agent")
        .env("RUST_LOG", "TRACE")
        .env("SMART_CONTRACT_ADDRESS", smart_contract_address)
        .env("TIME_TO_ACCUMULATE", "1")
        .kill_on_drop(true)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    ChildProcess { child }
}

#[tokio::test]
async fn test_general_flow() {
    eprintln!(); // to start test logs from new line

    // Start substrate contracts node and wait for its starting.
    let mut scn_child = start_substrate_contracts_node();
    let scn_stderr = scn_child.child.stderr.take().unwrap();
    let mut scn_reader = tokio::io::BufReader::new(scn_stderr);
    let (scn_s, scn_r) = oneshot::channel::<()>();
    tokio::spawn(async move {
        loop {
            let mut buf = String::new();
            scn_reader.read_line(&mut buf).await.unwrap();
            // By this line we understand that node is up and running.
            if buf.contains("Running JSON-RPC server") {
                break;
            }
        }
        scn_s.send(()).unwrap();
    });
    timeout(Duration::from_secs(10), scn_r).await.unwrap().unwrap();

    let smart_contract_address = deploy_smart_contract().await;
    eprintln!("Smart contract address {smart_contract_address}");

    let mut agent_child = start_agent(&smart_contract_address);
    let agent_stderr = agent_child.child.stderr.take().unwrap();
    let mut agent_reader = tokio::io::BufReader::new(agent_stderr);
    let (agent_s, agent_r) = oneshot::channel::<String>();
    tokio::spawn(async move {
        loop {
            let mut buf = String::new();
            agent_reader.read_line(&mut buf).await.unwrap();
            if buf.contains("starting tcp server on") {
                let parts: Vec<&str> = buf.split(" on ").collect();
                if parts.len() != 2 {
                    continue;
                }
                // Remove \n at the end.
                let agent_unix_socket = parts[1][..parts[1].len() - 1].to_string();
                agent_s.send(agent_unix_socket).unwrap();
                // Continue to read logs.
                loop {
                    let mut buf = String::new();
                    agent_reader.read_line(&mut buf).await.unwrap();
                    eprint!("Agent log: {buf}");
                }
            }
        }
    });
    let tcp_server_address = timeout(Duration::from_secs(10), agent_r).await.unwrap().unwrap();

    eprintln!(
        "Entity account id: {} - {:?}",
        subxt_signer::sr25519::dev::alice().public_key().to_account_id(),
        subxt_signer::sr25519::dev::alice().public_key().to_account_id().0
    );
    eprintln!(
        "Sub-entity account id: {} - {:?}",
        subxt_signer::sr25519::dev::bob().public_key().to_account_id(),
        subxt_signer::sr25519::dev::bob().public_key().to_account_id().0
    );

    let rpc_url = "ws://127.0.0.1:9944";
    let api = OnlineClient::<PolkadotConfig>::from_url(rpc_url).await.unwrap();

    let mut subscription = api.blocks().subscribe_finalized().await.unwrap();
    tokio::spawn(async move {
        loop {
            let block = subscription.next().await.unwrap().unwrap();
            let events = block.events().await.unwrap();
            process_event(events);
        }
    });
    // Wait 1s for block subscription logic and so ont.
    sleep(Duration::from_secs(2)).await;

    create_entity(&smart_contract_address);
    create_sub_entity(&smart_contract_address);

    // Store ok measurements to see certificate ready smart contract event.
    // We need to make some rpc requests to store measurements.
    // We cannot estimate exact number of rpc requests to reach certificate ready
    // smart contract ready.
    // Because our software (agent) accumulate several measurement before save
    // we do not know how many rpc requests we need to make.
    store_measurements(&api, &tcp_server_address, 6, 6).await;

    // Store measurement spikes to see too much spikes smart contract event.
    store_measurements(&api, &tcp_server_address, 69, 2).await;

    // todo: remove it after waiting for exact event by channel
    sleep(Duration::from_secs(3)).await;
}

fn create_entity(smart_contract_address: &str) {
    let res = std::process::Command::new("cargo")
        .args(vec![
            "contract",
            "call",
            "--contract",
            smart_contract_address,
            "--message",
            "create_entity",
            "--suri",
            "//Alice",
            "-x",
            "--skip-confirm",
        ])
        .current_dir("../emf_contract")
        .output()
        .unwrap();
    assert!(res.status.success());
}

fn create_sub_entity(smart_contract_address: &str) {
    let res = std::process::Command::new("cargo")
        .args(vec![
            "contract",
            "call",
            "--contract",
            smart_contract_address,
            "--message",
            "create_sub_entity",
            "--suri",
            "//Alice",
            "-x",
            "--skip-confirm",
            "--args",
            &format!(
                "\"{}\"",
                &subxt_signer::sr25519::dev::bob().public_key().to_account_id().to_string()
            ),
            "--args",
            "\"Berlin\"",
        ])
        .current_dir("../emf_contract")
        .output()
        .unwrap();
    assert!(res.status.success());
}

async fn rpc_store_measurement(tcp_server_address: &str, value: u128) {
    let mut stream = TcpStream::connect(tcp_server_address).await.unwrap();
    let mut buf = serde_json::to_vec(&RpcRequest { value }).unwrap();
    buf.push(b'\n');
    timeout(Duration::from_secs(3), stream.write_all(&buf)).await.unwrap().unwrap();
    let mut buf: Vec<u8> = Vec::new();
    timeout(Duration::from_secs(3), stream.read_to_end(&mut buf)).await.unwrap().unwrap();
    let res: RpcResponse = serde_json::from_slice(&buf).unwrap();
    assert_eq!(value, res.value);
}

async fn increase_block_timestamp(api: &OnlineClient<PolkadotConfig>) {
    // To increase block timestamp between measurements we need to make some transaction.
    // Otherwise timestamp on new measurement save will be the same
    // and we will get too fast revert error.
    let transfer_tx = api::tx()
        .balances()
        .transfer_allow_death(subxt_signer::sr25519::dev::alice().public_key().into(), 1);
    api.tx()
        .sign_and_submit_then_watch_default(&transfer_tx, &subxt_signer::sr25519::dev::bob())
        .await
        .unwrap()
        .wait_for_finalized()
        .await
        .unwrap();
}

fn process_event(events: Events<PolkadotConfig>) {
    let transcoder = ContractMessageTranscoder::load("assets/emf_contract.metadata.json").unwrap();
    // Topic hash to it's name.
    let mut topics: HashMap<String, String> = HashMap::new();
    for event_meta in transcoder.metadata().spec().events() {
        let topic = to_hex(event_meta.signature_topic().unwrap().as_bytes(), false);
        topics.insert(topic, event_meta.label().clone());
    }
    for event in events.iter().flatten() {
        if event.variant_name() != ContractEmitted::EVENT {
            continue;
        }
        // Usually first topic is our actual smart contract (EMF) event.
        let topic = event.topics()[0];
        let event = event.as_root_event::<RuntimeEvent>().unwrap();
        if let RuntimeEvent::Contracts(ContractsEvent::ContractEmitted { .. }) = event {
            eprintln!("NEW EVENT: {}", topics.get(&to_hex(&topic.0, false)).unwrap());
        }
    }
}

async fn store_measurements(
    api: &OnlineClient<PolkadotConfig>,
    tcp_server_address: &str,
    value: u128,
    count: usize,
) {
    for _ in 0..count {
        rpc_store_measurement(tcp_server_address, value).await;

        // To not make our transaction outdated after executing smart contract (store measurement).
        sleep(Duration::from_millis(1050)).await;
        increase_block_timestamp(api).await;

        // Wait some time before save new measurement to avoid too fast revert error.
        sleep(Duration::from_millis(1050)).await;
    }
}
