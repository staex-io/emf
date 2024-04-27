use std::{process::Stdio, str::from_utf8, time::Duration};

use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::oneshot,
    time::timeout,
};

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
            "30",
            "--args",
            "82800",
            "--args",
            "60",
            "--args",
            "360",
            "--args",
            "9",
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

fn start_agent() -> ChildProcess {
    let child = tokio::process::Command::new("../target/debug/agent")
        .env("RUST_LOG", "TRACE")
        .kill_on_drop(true)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    ChildProcess { child }
}

#[tokio::test]
async fn test() {
    eprintln!(); // to start test logs from new line

    // Start substrate contracts node and wait.
    let mut scn_child = start_substrate_contracts_node();
    let scn_stderr = scn_child.child.stderr.take().unwrap();
    let mut scn_reader = tokio::io::BufReader::new(scn_stderr);
    let (scn_s, scn_r) = oneshot::channel::<()>();
    tokio::spawn(async move {
        loop {
            let mut buf = String::new();
            scn_reader.read_line(&mut buf).await.unwrap();
            if buf.contains("Running JSON-RPC server") {
                break;
            }
        }
        scn_s.send(()).unwrap();
    });
    timeout(Duration::from_secs(10), scn_r).await.unwrap().unwrap();

    let _smart_contract_address = deploy_smart_contract().await;

    let mut agent_child = start_agent();
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
                return;
            }
        }
    });
    let tcp_server_address = timeout(Duration::from_secs(10), agent_r).await.unwrap().unwrap();

    let mut stream = TcpStream::connect(tcp_server_address).await.unwrap();
    const VALUE: u128 = 111;
    let mut buf = serde_json::to_vec(&RpcRequest { value: VALUE }).unwrap();
    buf.push(b'\n');
    timeout(Duration::from_secs(3), stream.write_all(&buf)).await.unwrap().unwrap();
    let mut buf: Vec<u8> = Vec::new();
    timeout(Duration::from_secs(3), stream.read_to_end(&mut buf)).await.unwrap().unwrap();
    let res: RpcResponse = serde_json::from_slice(&buf).unwrap();
    assert_eq!(VALUE, res.value);
}
