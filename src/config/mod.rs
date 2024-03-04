pub use config::*;
pub use server::*;
pub use upstream::*;
mod config;
mod reader;
mod server;
mod upstream;

pub fn is_default<T: Default + Eq>(val: &T) -> bool {
    *val == T::default()
}
