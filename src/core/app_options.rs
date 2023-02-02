use std::path::PathBuf;

use crate::prelude::*;
use crate::cli::app_args::{DebugArgs, GenerellArgs, CliArgs};

use super::app_config::AppConfig;
use super::file_access;

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

    pub fn get_data_path(&self) -> AppResult<PathBuf> {
        let data_path = if let Some(from_cli_env) = self.general().data_path() {
            debug!("Using path from cli or env var as data path");
            Ok(file_access::resolve_str_as_path(from_cli_env))
        } else {
            debug!("Using local data path from os");
            file_access::fetch_data_path(self)
        }?;

        info!("Using {:?} as data path", &data_path);

        Ok(data_path)
    }

    /// If true is return then the user local files are to be used even during development
    pub fn use_prod_local_share(&self) -> bool {
        self.get_debug_flag(|debug_args| debug_args.user_local_share_data())
            || cfg!(not(debug_assertions))
    }

    pub fn run_editor_dry(&self) -> bool {
        self.get_debug_flag(|debug_args| debug_args.run_editor_dry())
    }

    fn get_debug_flag(&self, getter: impl Fn(&DebugArgs) -> bool) -> bool {
        // In production never use it.
        if !cfg!(debug_assertions) {
            return false;
        }

        // In debug check if this alternative is desired.
        if let Some(use_it) = &self.debug {
            getter(use_it)
        } else {
            false
        }
    }
}
