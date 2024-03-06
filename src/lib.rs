#![allow(clippy::module_inception)]
#![allow(clippy::mutable_key_type)]
mod app_context;
pub mod blueprint;
pub mod builder;
pub mod config;
pub mod err_resp;
pub mod executor;
pub mod file_types;
pub mod http;
mod request;
mod response;
mod target_runtime;

pub use app_context::*;
pub use request::*;
pub use response::*;
pub use target_runtime::*;

#[async_trait::async_trait]
pub trait HttpIO: Sync + Send + 'static {
    async fn execute(
        &self,
        request: reqwest::Request,
        key: String,
    ) -> anyhow::Result<Response<bytes::Bytes>>;
}

#[async_trait::async_trait]
pub trait FileIO: Send + Sync {
    async fn write<'a>(&'a self, path: &'a str, content: &'a [u8], key: String) -> anyhow::Result<()>;
    async fn read<'a>(&'a self, path: &'a str, key: String) -> anyhow::Result<String>;
}
