use std::sync::Arc;
use tokio::sync::oneshot;
use crate::server::server_config::ServerConfig;

pub async fn start_http_1(
    sc: Arc<ServerConfig>,
    server_up_sender: Option<oneshot::Sender<()>>,
) -> anyhow::Result<()> {
    super::log_launch_and_open_browser(sc.as_ref());
    
    Ok(())
}