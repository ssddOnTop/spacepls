pub use config::*;
pub use server::*;
pub use upstream::*;
mod config;
mod extensions;
pub mod reader;
mod server;
mod upstream;

pub fn is_default<T: Default + Eq>(val: &T) -> bool {
    *val == T::default()
}
