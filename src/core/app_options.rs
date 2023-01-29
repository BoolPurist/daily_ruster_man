/// Used in the core modules to alter certain behaviors of app
use crate::cli::app_args::{DebugArgs, GenerellArgs, CliArgs};
#[derive(Getters)]
pub struct AppOptions {
    /// During development, this will have some value.
    debug: Option<DebugArgs>,
    #[getset(get = "pub")]
    generell: GenerellArgs,
}

impl AppOptions {
    pub fn new(args: &CliArgs) -> Self {
        let generell = args.args().clone();
        if cfg!(debug_assertions) {
            Self {
                generell,
                debug: Some(args.debug_args().clone()),
            }
        } else {
            Self {
                generell,
                debug: None,
            }
        }
    }

    /// If true is return then the user local files are to be used even during development
    pub fn use_prod_local_share(&self) -> bool {
        if let Some(use_it) = &self.debug {
            use_it.user_local_share()
        } else {
            false
        }
    }
}
