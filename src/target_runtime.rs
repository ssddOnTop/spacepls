use crate::{FileIO, HttpIO};
use std::sync::Arc;

#[derive(Clone)]
/// The TargetRuntime struct unifies the available runtime-specific
/// IO implementations. This is used to reduce piping IO structs all
/// over the codebase.
pub struct TargetRuntime {
    pub http: Arc<dyn HttpIO>,
    pub file: Arc<dyn FileIO>,
}
