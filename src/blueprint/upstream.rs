use std::collections::BTreeSet;

use derive_setters::Setters;

use crate::config::{self, Batch, Config};
use anyhow::Result;

#[derive(PartialEq, Eq, Clone, Debug, schemars::JsonSchema)]
pub struct Proxy {
    pub url: String,
}

#[derive(PartialEq, Eq, Clone, Debug, Setters, schemars::JsonSchema)]
pub struct Upstream {
    pub pool_idle_timeout: u64,
    pub pool_max_idle_per_host: usize,
    pub keep_alive_interval: u64,
    pub keep_alive_timeout: u64,
    pub keep_alive_while_idle: bool,
    pub proxy: Option<Proxy>,
    pub connect_timeout: u64,
    pub timeout: u64,
    pub tcp_keep_alive: u64,
    pub user_agent: String,
    pub allowed_headers: BTreeSet<String>,
    pub http_cache: bool,
    pub batch: Option<Batch>,
}

impl Upstream {
    pub fn is_batching_enabled(&self) -> bool {
        if let Some(batch) = self.batch.as_ref() {
            batch.delay >= 1 || batch.max_size >= 1
        } else {
            false
        }
    }
}

impl Default for Upstream {
    fn default() -> Self {
        // NOTE: Using unwrap because try_from default will never fail
        Upstream::try_from(&Config::default()).unwrap()
    }
}

impl TryFrom<&Config> for Upstream {
    type Error = anyhow::Error;

    fn try_from(config: &Config) -> Result<Self, Self::Error> {
        let config_upstream = &config.upstream;
        let batch = get_batch(config_upstream);
        let proxy = get_proxy(config_upstream);
        let upstream = Upstream {
            pool_idle_timeout: (config_upstream).get_pool_idle_timeout(),
            pool_max_idle_per_host: (config_upstream).get_pool_max_idle_per_host(),
            keep_alive_interval: (config_upstream).get_keep_alive_interval(),
            keep_alive_timeout: (config_upstream).get_keep_alive_timeout(),
            keep_alive_while_idle: (config_upstream).get_keep_alive_while_idle(),
            proxy,
            connect_timeout: config_upstream.get_connect_timeout(),
            timeout: config_upstream.get_timeout(),
            tcp_keep_alive: config_upstream.get_tcp_keep_alive(),
            user_agent: config_upstream.get_user_agent(),
            allowed_headers: config_upstream.get_allowed_headers(),
            http_cache: config_upstream.get_enable_http_cache(),
            batch,
        };

        Ok(upstream)
    }
}

fn get_batch(upstream: &config::Upstream) -> Option<Batch> {
    upstream.batch.as_ref().map(|batch| Batch {
        max_size: upstream.get_max_size(),
        delay: upstream.get_delay(),
        headers: batch.headers.clone(),
    })
}

fn get_proxy(upstream: &config::Upstream) -> Option<Proxy> {
    upstream.proxy.as_ref().map(|proxy| Proxy { url: proxy.url.clone() })
}
