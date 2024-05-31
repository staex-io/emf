use std::{process::Stdio, str::from_utf8, time::Duration};

use rand::{distributions::uniform::SampleRange, rngs::ThreadRng, Rng};
use serde::{Deserialize, Serialize};
use subxt::{utils::AccountId32, OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::Keypair;
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::oneshot,
    time::{sleep, timeout},
};

mod emf_contract;
mod http_api;

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

async fn deploy_smart_contract(is_drift_mode: bool) -> String {
    tokio::process::Command::new("cargo")
        .args(vec!["contract", "upload", "--suri", "//Alice", "-x"])
        .kill_on_drop(true)
        .current_dir("../emf_contract")
        .output()
        .await
        .unwrap();

    // So in drift mode we don't need to increase block timestamp.
    // To setup our environment faster.
    let min_time_between_measurements_to_save = if is_drift_mode { "0" } else { "1" };

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
            min_time_between_measurements_to_save,
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
        .args(vec!["run"])
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
    let mut rng = rand::thread_rng();
    let is_drift_mode = std::env::var("DRIFT_MODE").is_ok();

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

    let smart_contract_address = deploy_smart_contract(is_drift_mode).await;
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

    let entity = subxt_signer::sr25519::dev::alice();
    let sub_entity = subxt_signer::sr25519::dev::bob();

    eprintln!(
        "Entity account id: {} - {:?}",
        entity.public_key().to_account_id(),
        entity.public_key().to_account_id().0
    );
    eprintln!(
        "Sub-entity account id: {} - {:?}",
        sub_entity.public_key().to_account_id(),
        sub_entity.public_key().to_account_id().0
    );

    let rpc_url = "ws://127.0.0.1:9944";
    let api = OnlineClient::<PolkadotConfig>::from_url(rpc_url).await.unwrap();

    create_entity(&smart_contract_address);

    if is_drift_mode {
        eprintln!("Starting drift mode...");
        prepare_drift_data(&smart_contract_address, &mut rng).await;
        sleep(Duration::from_secs(60 * 60)).await;
    }

    create_sub_entity(
        &smart_contract_address,
        &sub_entity.public_key().to_account_id(),
        "52.6443,13.0792",
    );

    // Store ok measurements to see certificate ready smart contract event.
    // We need to make some rpc requests to store measurements.
    // We cannot estimate exact number of rpc requests to reach certificate ready
    // smart contract ready.
    // Because our software (agent) accumulate several measurement before save
    // we do not know how many rpc requests we need to make.
    store_measurements(6, 1..11, &entity, &sub_entity, &api, &tcp_server_address, &mut rng).await;

    // Store measurement spikes to see too many spikes smart contract event.
    store_measurements(2, 11..100, &entity, &sub_entity, &api, &tcp_server_address, &mut rng).await;

    timeout(
        Duration::from_secs(10),
        wait_for_events(
            &smart_contract_address,
            entity.public_key().to_account_id(),
            sub_entity.public_key().to_account_id(),
        ),
    )
    .await
    .unwrap();
}

async fn prepare_drift_data(smart_contract_address: &str, rng: &mut ThreadRng) {
    // Charlie has good certificate.
    let charlie = subxt_signer::sr25519::dev::charlie();
    create_sub_entity(
        smart_contract_address,
        &charlie.public_key().to_account_id(),
        "52.5805,13.3738",
    );
    // Dave has bad certificate.
    let dave = subxt_signer::sr25519::dev::dave();
    create_sub_entity(
        smart_contract_address,
        &dave.public_key().to_account_id(),
        "52.6539,13.6093",
    );
    // Eve doesn't have certificate.
    let eve = subxt_signer::sr25519::dev::eve();
    create_sub_entity(smart_contract_address, &eve.public_key().to_account_id(), "52.3643,13.5043");
    // Ferdie doesn't have enough records.
    let ferdie = subxt_signer::sr25519::dev::ferdie();
    create_sub_entity(
        smart_contract_address,
        &ferdie.public_key().to_account_id(),
        "52.3966,13.0593",
    );

    prepare_drift_tower(smart_contract_address, "//Charlie", rng, 1..11, 2);
    prepare_drift_tower(smart_contract_address, "//Dave", rng, 11..100, 2);
    prepare_drift_tower(smart_contract_address, "//Eve", rng, 0..100, 2);
    prepare_drift_tower(smart_contract_address, "//Ferdie", rng, 0..100, 1);

    issue_certificate(smart_contract_address, &charlie.public_key().to_account_id());
    issue_certificate(smart_contract_address, &dave.public_key().to_account_id());
}

fn prepare_drift_tower<R>(
    smart_contract_address: &str,
    sub_entity_suri: &str,
    rng: &mut ThreadRng,
    range: R,
    count: usize,
) where
    R: SampleRange<u128> + Clone,
{
    for _ in 0..count {
        store_measurement(smart_contract_address, sub_entity_suri, rng.gen_range(range.clone()));
    }
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

fn create_sub_entity(smart_contract_address: &str, sub_entity: &AccountId32, location: &str) {
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
            &format!("\"{}\"", sub_entity),
            "--args",
            &format!("\"{}\"", location),
        ])
        .current_dir("../emf_contract")
        .output()
        .unwrap();
    assert!(res.status.success());
}

fn store_measurement(smart_contract_address: &str, sub_entity_suri: &str, value: u128) {
    let res = std::process::Command::new("cargo")
        .args(vec![
            "contract",
            "call",
            "--contract",
            smart_contract_address,
            "--message",
            "store_measurement",
            "--suri",
            sub_entity_suri,
            "-x",
            "--skip-confirm",
            "--args",
            &format!("\"{}\"", value),
        ])
        .current_dir("../emf_contract")
        .output()
        .unwrap();
    assert!(res.status.success());
}

fn issue_certificate(smart_contract_address: &str, sub_entity: &AccountId32) {
    let res = std::process::Command::new("cargo")
        .args(vec![
            "contract",
            "call",
            "--contract",
            smart_contract_address,
            "--message",
            "issue_certificate",
            "--suri",
            "//Alice",
            "-x",
            "--skip-confirm",
            "--args",
            &format!("\"{}\"", sub_entity),
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

async fn increase_block_timestamp(
    api: &OnlineClient<PolkadotConfig>,
    entity: &Keypair,
    sub_entity: &Keypair,
) {
    // To increase block timestamp between measurements we need to make some transaction.
    // Otherwise timestamp on new measurement save will be the same
    // and we will get too fast revert error.
    let transfer_tx =
        emf_contract::api::tx().balances().transfer_allow_death(entity.public_key().into(), 1);
    api.tx()
        .sign_and_submit_then_watch_default(&transfer_tx, sub_entity)
        .await
        .unwrap()
        .wait_for_finalized()
        .await
        .unwrap();
}

async fn store_measurements<R>(
    count: usize,
    range: R,
    entity: &Keypair,
    sub_entity: &Keypair,
    api: &OnlineClient<PolkadotConfig>,
    tcp_server_address: &str,
    rng: &mut ThreadRng,
) where
    R: SampleRange<u128> + Clone,
{
    for _ in 0..count {
        rpc_store_measurement(tcp_server_address, rng.gen_range(range.clone())).await;

        // To not make our transaction outdated after executing smart contract (store measurement).
        sleep(Duration::from_millis(1050)).await;
        increase_block_timestamp(api, entity, sub_entity).await;

        // Wait some time before save new measurement to avoid too fast revert error.
        sleep(Duration::from_millis(1050)).await;
    }
}

async fn wait_for_events(
    smart_contract_address: &str,
    entity: AccountId32,
    sub_entity: AccountId32,
) {
    let expected_sub_entity = sub_entity.clone().to_string();
    let mut issued = false;
    loop {
        eprintln!("new iteration of waiting events by http api");

        let entities = http_api::request_entities().await;
        if entities.is_empty() {
            sleep(Duration::from_secs(1)).await;
            continue;
        }
        assert_eq!(entity.to_string(), entities[0].account_id);

        let sub_entities = http_api::request_sub_entities(&entity).await;
        if sub_entities.is_empty() {
            sleep(Duration::from_secs(1)).await;
            continue;
        }
        assert_eq!(entity.to_string(), sub_entities[0].entity);
        assert_eq!(expected_sub_entity, sub_entities[0].account_id);

        let spikes = http_api::request_spikes(&sub_entity).await;
        if spikes.len() != 2 {
            sleep(Duration::from_secs(1)).await;
            continue;
        }
        assert_eq!(expected_sub_entity, spikes[0].sub_entity);
        assert_eq!(expected_sub_entity, spikes[1].sub_entity);
        assert!(spikes[0].value.parse::<u128>().unwrap() >= 11);
        assert!(spikes[1].value.parse::<u128>().unwrap() >= 11);

        let too_many_spikes = http_api::request_too_many_spikes(&sub_entity).await;
        if too_many_spikes.is_empty() {
            sleep(Duration::from_secs(1)).await;
            continue;
        }
        assert_eq!(expected_sub_entity, too_many_spikes[0].sub_entity);

        let ready_certificates = http_api::request_ready_certificates(&sub_entity).await;
        if ready_certificates.is_empty() {
            sleep(Duration::from_secs(1)).await;
            continue;
        }
        assert_eq!(expected_sub_entity, ready_certificates[0].sub_entity);

        if !issued {
            issued = true;
            issue_certificate(smart_contract_address, &sub_entity);
            continue;
        }

        let issued_certificates = http_api::request_issued_certificates(&sub_entity).await;
        if issued_certificates.is_empty() {
            sleep(Duration::from_secs(1)).await;
            continue;
        }
        assert_eq!(expected_sub_entity, issued_certificates[0].sub_entity);

        return;
    }
}
