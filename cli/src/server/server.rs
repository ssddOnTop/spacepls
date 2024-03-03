use std::sync::Arc;

use anyhow::Result;
use tokio::sync::oneshot::{self};

use super::http1::start_http_1;
use super::server_config::ServerConfig;
use spacepls::blueprint::Blueprint;
use spacepls::config::Config;

pub struct Server {
    config: Config,
    server_up_sender: Option<oneshot::Sender<()>>,
}

impl Server {
    pub fn new(config: Config) -> Self {
        Self { config, server_up_sender: None }
    }

    pub fn server_up_receiver(&mut self) -> oneshot::Receiver<()> {
        let (tx, rx) = oneshot::channel();

        self.server_up_sender = Some(tx);

        rx
    }

    /// Starts the server in the current Runtime
    pub async fn start(self) -> Result<()> {
        let blueprint = Blueprint::try_from(&self.config)?;
        let server_config = Arc::new(ServerConfig::new(blueprint.clone()));

        start_http_1(server_config, self.server_up_sender).await
    }

    /// Starts the server in its own multithreaded Runtime
    pub async fn fork_start(self) -> Result<()> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(self.config.server.get_workers())
            .enable_all()
            .build()?;

        let result = runtime.spawn(self.start()).await?;
        runtime.shutdown_background();

        result
    }
}
