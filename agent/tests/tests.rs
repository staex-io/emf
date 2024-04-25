use std::{process::Stdio, str::from_utf8, time::Duration};

use tokio::{io::AsyncBufReadExt, sync::oneshot, time::timeout};

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
        .spawn()
        .unwrap();
    ChildProcess { child }
}

#[tokio::test]
async fn test_everything() {
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

    let _agent_child = start_agent();
}
