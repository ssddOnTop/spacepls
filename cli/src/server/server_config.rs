use std::net::SocketAddr;
use std::sync::Arc;
use spacepls::AppContext;
use spacepls::blueprint::Blueprint;

use crate::runtime;

pub struct ServerConfig {
    pub app_ctx: Arc<AppContext>,
}

impl ServerConfig {
    pub fn playground_url(&self) -> String {
        self.addr().to_string()
    }
}

impl ServerConfig {
    pub fn new(blueprint: Blueprint) -> Self {
        let app_ctx = AppContext {
            runtime: runtime::init(&blueprint.upstream),
            blueprint,
        };
        let app_ctx = Arc::new(app_ctx);
        Self { app_ctx }
    }

    pub fn addr(&self) -> SocketAddr {
        (self.app_ctx.blueprint.server.hostname, self.app_ctx.blueprint.server.port).into()
    }
}
