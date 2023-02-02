use std::path::PathBuf;

use crate::core::constants::SIGN_FOR_FROM_CONF_FOLDER;
use crate::core::file_access;

use super::AppConfig;

pub struct PatchFromConfig(Option<String>);

impl PatchFromConfig {
    pub fn new(path: Option<String>) -> Self {
        Self(path)
    }
    pub fn try_to_resolved_path(&self, option: &AppConfig) -> Option<PathBuf> {
        self.0.as_ref().map(|path| {
            if path.starts_with(SIGN_FOR_FROM_CONF_FOLDER) {
                let without_plus = path
                    .strip_prefix(SIGN_FOR_FROM_CONF_FOLDER)
                    .expect("Unexpexted: check before ensured there is + to remove from the left");
                option.root_path().join(without_plus)
            } else {
                file_access::resolve_str_as_path(path)
            }
        })
    }
}
