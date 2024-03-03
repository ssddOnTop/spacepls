use std::net::IpAddr;
use derive_setters::Setters;
use crate::config::Config;
use anyhow::Result;

#[derive(Clone, Debug, Setters)]
pub struct Server {
    pub hostname: IpAddr,
    pub port: u16,
    pub workers: usize,
}

impl TryFrom<&Config> for Server {
    type Error = anyhow::Error;

    fn try_from(config: &Config) -> Result<Self, Self::Error> {
        let config_server = config.server.clone();
        let hostname = validate_hostname(config_server.get_hostname())?;
        let port = config_server.get_port();
        let workers = config_server.get_workers();
        Ok(
            Self {
                hostname,
                port,
                workers,
            }
        )
    }
}

fn validate_hostname(hostname: String) -> Result<IpAddr> {
    if hostname == "localhost" {
        Ok(IpAddr::from([127, 0, 0, 1]))
    } else {
        Ok(hostname.parse()?)
    }
}