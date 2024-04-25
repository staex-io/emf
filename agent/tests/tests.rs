use std::{process::Stdio, time::Duration};

use tokio::{io::AsyncBufReadExt, sync::oneshot, time::timeout};

struct ChildProcess {
    child: tokio::process::Child,
}

struct SubstrateContractsNode {}

impl SubstrateContractsNode {
    fn start() -> ChildProcess {
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
}

#[tokio::test]
async fn test_everything() {
    eprintln!(); // to start test logs from new line

    // Start substrate contracts node and wait.
    let mut scn_child = SubstrateContractsNode::start();
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
}
