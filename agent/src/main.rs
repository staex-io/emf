use std::fmt::Debug;
use std::time::Duration;

use log::{debug, error, info, trace, LevelFilter};
use tokio::net::unix::SocketAddr;
use tokio::net::UnixStream;
use tokio::{net::UnixListener, select, sync::watch, time::timeout};

mod emf_contract;

type Res<T> = Result<T, Error>;

struct Error(String);

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

#[tokio::main]
async fn main() -> Res<()> {
    env_logger::builder()
        .filter(None, LevelFilter::Off)
        .filter_module("agent", LevelFilter::Trace)
        .init();
    let (stop_s, stop_r) = watch::channel(());
    tokio::spawn(async move { start_unix_server(stop_r).await });
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

async fn start_unix_server(mut stop_r: watch::Receiver<()>) {
    let listener = UnixListener::bind("").unwrap();
    loop {
        select! {
            _ = stop_r.changed() => {
                trace!("received stop signal, exit unix server loop");
                return;
            }
            connection = listener.accept() => {
                if let Ok(connection) = connection {
                    tokio::spawn(async move {
                        if let Err(e) = process_connection(connection).await {
                            error!("failed to process connection: {}", e.0)
                        }
                    });
                }
            }
        }
    }
}

async fn process_connection(connection: (UnixStream, SocketAddr)) -> Res<()> {
    trace!("acquired new unix socket connection");
    let (_stream, _) = connection;
    Ok(())
}
