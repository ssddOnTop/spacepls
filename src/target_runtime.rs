use std::sync::Arc;
use crate::{FileIO, HttpIO};

#[derive(Clone)]
/// The TargetRuntime struct unifies the available runtime-specific
/// IO implementations. This is used to reduce piping IO structs all
/// over the codebase.
pub struct TargetRuntime {
    pub http: Arc<dyn HttpIO>,
    pub file: Arc<dyn FileIO>,
}
