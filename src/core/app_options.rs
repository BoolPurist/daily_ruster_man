/// Used in the core modules to alter certain behaviors
use crate::cli::app_args::{DebugArgs, GenerellArgs, CliArgs};
#[derive(Getters)]
pub struct AppOptions {
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

    pub fn use_prod_local_share(&self) -> bool {
        if let Some(use_it) = &self.debug {
            use_it.user_local_share()
        } else {
            false
        }
    }
}
