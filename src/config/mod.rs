pub use config::*;
pub use upstream::*;
pub use server::*;
mod server;
mod config;
mod upstream;
mod reader;

pub fn is_default<T: Default + Eq>(val: &T) -> bool {
    *val == T::default()
}