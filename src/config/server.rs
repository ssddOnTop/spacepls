use serde::{Deserialize, Serialize};
use super::is_default;

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
/// `Server` is the configuration for the SpacePls server.
pub struct Server {
    #[serde(default, skip_serializing_if = "is_default")]
    /// `hostname` sets the server hostname.
    pub hostname: Option<String>,
    #[serde(default, skip_serializing_if = "is_default")]
    /// `port` sets the SpacePls running port. @default `8000`.
    pub port: Option<u16>,
    #[serde(default, skip_serializing_if = "is_default")]
    /// `workers` set the number of worker threads. @default is one thread.
    pub workers: Option<usize>,
}

impl Server {
    pub fn get_hostname(&self) -> String {
        self.hostname.clone().unwrap_or_else(|| "127.0.0.1".to_string())
    }
    pub fn get_port(&self) -> u16 {
        self.port.unwrap_or(19194)
    }
    pub fn get_workers(&self) -> usize {
        self.workers.unwrap_or(1)
    }
    pub fn merge_right(mut self, other: Self) -> Self {
        self.workers = other.workers.or(self.workers);
        self.port = other.port.or(self.port);
        self.hostname = other.hostname.or(self.hostname);
        self
    }
}