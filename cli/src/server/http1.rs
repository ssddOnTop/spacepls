use crate::server::server_config::ServerConfig;
use hyper::body::Incoming;
use hyper::service::service_fn;
use hyper::Request;
use spacepls::http::handle_request;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::oneshot;

pub async fn start_http_1(
    sc: Arc<ServerConfig>,
    server_up_sender: Option<oneshot::Sender<()>>,
) -> anyhow::Result<()> {
    super::log_launch_and_open_browser(sc.as_ref());
    run(sc, server_up_sender).await
}

async fn run(
    sc: Arc<ServerConfig>,
    server_up_sender: Option<oneshot::Sender<()>>,
) -> anyhow::Result<()> {
    let addr = sc.addr();
    let listener = TcpListener::bind(addr).await?;
    if let Some(sender) = server_up_sender {
        sender
            .send(())
            .or(Err(anyhow::anyhow!("Failed to send message")))?;
    }
    loop {
        let stream_result = listener.accept().await;
        match stream_result {
            Ok((stream, _)) => {
                let io = hyper_util::rt::TokioIo::new(stream);
                let sc = sc.clone();
                tokio::spawn(async move {
                    let server = hyper::server::conn::http1::Builder::new()
                        .serve_connection(
                            io,
                            service_fn(move |req: Request<Incoming>| {
                                let state = sc.clone();
                                async move {
                                    handle_request(req, state.app_ctx.clone()).await
                                }
                            }),
                        )
                        .await;
                    if let Err(e) = server {
                        log::error!("An error occurred while handling a request: {e}");
                    }
                });
            }
            Err(e) => log::error!("An error occurred while handling request: {e}"),
        }
    }
}
