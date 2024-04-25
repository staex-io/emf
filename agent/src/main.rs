use std::fmt::Debug;
use std::time::Duration;

use log::{debug, error, info, LevelFilter};
use tokio::{sync::watch, time::timeout};

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
    let (stop_s, mut stop_r) = watch::channel(());
    tokio::spawn(async move {
        if let Err(e) = stop_r.changed().await {
            error!("failed to received stop signal: {e}")
        }
    });
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
