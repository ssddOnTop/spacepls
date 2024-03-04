pub mod builder;
pub mod executor;
mod response;
mod request;
pub mod err_resp;
mod target_runtime;
pub mod file_types;
pub mod config;
pub mod blueprint;
mod app_context;
pub mod http;

pub use target_runtime::*;
pub use request::*;
pub use response::*;
pub use app_context::*;

#[async_trait::async_trait]
pub trait HttpIO: Sync + Send + 'static {
    async fn execute(
        &self,
        request: reqwest::Request,
    ) -> anyhow::Result<response::Response<bytes::Bytes>>;
}

#[async_trait::async_trait]
pub trait FileIO: Send + Sync {
    async fn write<'a>(&'a self, path: &'a str, content: &'a [u8]) -> anyhow::Result<()>;
    async fn read<'a>(&'a self, path: &'a str) -> anyhow::Result<String>;
}