use derive_setters::Setters;
use serde::{Deserialize, Serialize};

#[derive(
    Serialize, Deserialize, Clone, Debug, Default, Setters, PartialEq, Eq, schemars::JsonSchema,
)]
/// Used to store crucial information like
/// password and dir path
pub struct Extensions {
    /// The directory path to store files.
    #[serde(default)]
    pub dir_path: Option<String>,
    /// Password
    #[serde(default)]
    pub password: Option<String>,
}

impl Extensions {
    pub(crate) fn merge_right(self, other: Extensions) -> Extensions {
        let dir_path = other.clone().dir_path.or(self.dir_path);
        let password = other.clone().password.or(self.password);
        Self { dir_path, password }
    }
}
