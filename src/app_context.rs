use crate::blueprint::Blueprint;
use crate::TargetRuntime;

pub struct AppContext {
    pub blueprint: Blueprint,
    pub runtime: TargetRuntime,
}