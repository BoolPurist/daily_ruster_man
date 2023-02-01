pub mod option_sources;
use crate::prelude::*;
use crate::cli::app_args::{DebugArgs, GenerellArgs, CliArgs};

use super::app_config::AppConfig;

pub fn get_from_env(name: &str) -> Option<String> {
    std::env::var(name).ok()
}

/// Used in the core modules to alter certain behaviors of app
#[derive(Getters)]
pub struct AppOptions {
    /// During development, this will have some value.
    debug: Option<DebugArgs>,
    #[getset(get = "pub")]
    general: GenerellArgs,
    config: OnceCell<Option<AppConfig>>,
}

impl AppOptions {
    fn init(general: GenerellArgs, debug: Option<DebugArgs>) -> Self {
        Self {
            config: Default::default(),
            general,
            debug,
        }
    }
    #[cfg(debug_assertions)]
    pub fn new(args: &CliArgs) -> Self {
        Self::init(args.args().clone(), Some(args.debug_args().clone()))
    }

    #[cfg(not(debug_assertions))]
    pub fn new(args: &CliArgs) -> Self {
        Self::init(args.args().clone(), None)
    }

    pub fn load_config(&self) -> AppResult<Option<&AppConfig>> {
        self.config
            .get_or_try_init(|| AppConfig::try_from_file_system(self))
            .map(|to_ref| to_ref.as_ref())
    }

    /// If true is return then the user local files are to be used even during development
    pub fn use_prod_local_share(&self) -> bool {
        // In production always use local files of user.
        if !cfg!(debug_assertions) {
            return true;
        }

        // In debug the local files of user should only be used if the option was given.
        if let Some(use_it) = &self.debug {
            use_it.user_local_share()
        } else {
            false
        }
    }
}
