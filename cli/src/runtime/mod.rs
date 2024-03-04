mod file;
mod http;

use std::sync::Arc;

use spacepls::blueprint;
use spacepls::TargetRuntime;

// Provides access to http in native rust environment
pub fn init(upstream: &blueprint::Upstream) -> TargetRuntime {
    TargetRuntime {
        http: Arc::new(http::NativeHttp::init(upstream)),
        file: Arc::new(file::NativeFileIO::init()),
    }
}
