use crate::config::server::Server;
use crate::config::Upstream;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};

#[derive(
    Serialize, Deserialize, Clone, Debug, Default, Setters, PartialEq, Eq, schemars::JsonSchema,
)]
pub struct Config {
    ///
    /// Describes how SpacePls server should behave.
    ///
    #[serde(default)]
    pub server: Server,
    ///
    /// Dictates how SpacePls should handle upstream requests/responses.
    /// Tuning upstream can improve performance and reliability for connections.
    ///
    #[serde(default)]
    pub upstream: Upstream,

    /// The directory path to store files.
    #[serde(default)]
    pub dir_path: Option<String>,
    /// Password
    #[serde(default)]
    pub password: Option<String>,
}

impl Config {
    /// Merge configs, preferring the right side.
    pub fn merge_right(self, other: &Self) -> Self {
        let server = self.server.merge_right(other.server.clone());
        let upstream = self.upstream.merge_right(other.upstream.clone());
        let dir_path = other.clone().dir_path.or(self.dir_path);
        let password = other.clone().password.or(self.password);

        Self {
            server,
            upstream,
            dir_path,
            password,
        }
    }

    /// Serialize Config.
    pub fn from_json(json: &str) -> anyhow::Result<Self> {
        Ok(serde_json::from_str(json)?)
    }
}
