use crate::config::extensions::Extensions;
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

    /// Used to store crucial information like
    /// password and dir path
    pub extensions: Extensions,
}

impl Config {
    /// Merge configs, preferring the right side.
    pub fn merge_right(self, other: &Self) -> Self {
        let server = self.server.merge_right(other.server.clone());
        let upstream = self.upstream.merge_right(other.upstream.clone());
        let extensions = self.extensions.merge_right(other.extensions.clone());

        Self {
            server,
            upstream,
            extensions,
        }
    }

    /// Serialize Config.
    pub fn from_json(json: &str) -> anyhow::Result<Self> {
        Ok(serde_json::from_str(json)?)
    }
}
