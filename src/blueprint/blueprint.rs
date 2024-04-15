use super::Upstream;
use crate::blueprint::server::Server;
use crate::config::Config;
use anyhow::Context;
use derive_setters::Setters;

#[derive(Clone, Debug, Setters)]
pub struct Blueprint {
    pub server: Server,
    pub upstream: Upstream,
    pub dir_path: String,
}

impl TryFrom<&Config> for Blueprint {
    type Error = anyhow::Error;
    fn try_from(config: &Config) -> Result<Self, Self::Error> {
        let server = Server::try_from(config)?;
        let upstream = Upstream::try_from(config)?;
        let dir_path =
            config.extensions.dir_path.clone().context(
                "No dir path found in config, use config reader to read config instead.",
            )?;
        Ok(Self {
            server,
            upstream,
            dir_path,
        })
    }
}
