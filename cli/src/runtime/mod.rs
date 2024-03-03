mod file;
mod http;

use std::hash::Hash;
use std::sync::Arc;

use spacepls::TargetRuntime;
use spacepls::{blueprint, FileIO, HttpIO};

// Provides access to file system in native rust environment
fn init_file() -> Arc<dyn FileIO> {
    Arc::new(file::NativeFileIO::init())
}

// Provides access to http in native rust environment
fn init_http(upstream: &blueprint::Upstream) -> Arc<dyn HttpIO> {
    let http_io = http::NativeHttp::init(upstream);
    Arc::new(http_io)
}

// Provides access to http in native rust environment
pub fn init(upstream: &blueprint::Upstream) -> TargetRuntime {
    TargetRuntime {
        http: init_http(upstream),
        file: init_file(),
    }
}
