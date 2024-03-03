use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use spacepls::blueprint::Blueprint;

use crate::runtime;

pub struct ServerConfig {
    pub blueprint: Blueprint,
    pub rt: Arc<spacepls::TargetRuntime>,
}

impl ServerConfig {
    pub fn new(blueprint: Blueprint) -> Self {
        let rt = Arc::new(runtime::init(&blueprint.upstream));
        Self { blueprint, rt }
    }

    pub fn addr(&self) -> SocketAddr {
        (self.blueprint.server.hostname, self.blueprint.server.port).into()
    }
}
