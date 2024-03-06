use std::time::Duration;

use anyhow::Result;
use http_cache_reqwest::{Cache, CacheMode, HttpCache, HttpCacheOptions, MokaManager};
use hyper::body::Bytes;
use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use spacepls::blueprint::Upstream;
use spacepls::HttpIO;
use spacepls::Response;

#[derive(Clone)]
pub struct NativeHttp {
    client: ClientWithMiddleware,
}

impl Default for NativeHttp {
    fn default() -> Self {
        Self {
            client: ClientBuilder::new(Client::new()).build(),
        }
    }
}

impl NativeHttp {
    pub fn init(upstream: &Upstream) -> Self {
        let mut builder = Client::builder()
            .tcp_keepalive(Some(Duration::from_secs(upstream.tcp_keep_alive)))
            .timeout(Duration::from_secs(upstream.timeout))
            .connect_timeout(Duration::from_secs(upstream.connect_timeout))
            .http2_keep_alive_interval(Some(Duration::from_secs(upstream.keep_alive_interval)))
            .http2_keep_alive_timeout(Duration::from_secs(upstream.keep_alive_timeout))
            .http2_keep_alive_while_idle(upstream.keep_alive_while_idle)
            .pool_idle_timeout(Some(Duration::from_secs(upstream.pool_idle_timeout)))
            .pool_max_idle_per_host(upstream.pool_max_idle_per_host)
            .user_agent(upstream.user_agent.clone());

        // Add Http Proxy
        if let Some(ref proxy) = upstream.proxy {
            builder = builder.proxy(
                reqwest::Proxy::http(proxy.url.clone())
                    .expect("Failed to set proxy in http client"),
            );
        }

        let mut client = ClientBuilder::new(builder.build().expect("Failed to build client"));

        if upstream.http_cache {
            client = client.with(Cache(HttpCache {
                mode: CacheMode::Default,
                manager: MokaManager::default(),
                options: HttpCacheOptions::default(),
            }))
        }
        Self {
            client: client.build(),
        }
    }
}

#[async_trait::async_trait]
impl HttpIO for NativeHttp {
    async fn execute(&self, request: reqwest::Request, _: String) -> Result<Response<Bytes>> {
        // TODO add AES support
        log::info!(
            "{} {} {:?}",
            request.method(),
            request.url(),
            request.version()
        );
        log::debug!("request: {:?}", request);
        let response = self.client.execute(request).await;
        log::debug!("response: {:?}", response);
        Ok(Response::from_reqwest(response?.error_for_status()?).await?)
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Method;

    use super::*;

    fn start_mock_server() -> httpmock::MockServer {
        httpmock::MockServer::start()
    }

    #[tokio::test]
    async fn test_native_http_get_request() {
        let server = start_mock_server();

        let header_serv = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/test");
            then.status(200).body("Alo");
        });

        let native_http = NativeHttp::init(&Default::default());
        let port = server.port();
        // Build a GET request to the mock server
        let request_url = format!("http://localhost:{}/test", port);
        let request = reqwest::Request::new(Method::GET, request_url.parse().unwrap());

        // Execute the request
        let result = native_http.execute(request, String::new()).await;

        // Assert the response is as expected
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status, reqwest::StatusCode::OK);
        assert_eq!(response.body, Bytes::from("Alo"));

        header_serv.assert();
    }
}
