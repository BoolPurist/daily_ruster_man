use crate::core::{constants::DEFAUTL_EDITOR, app_options::AppOptions};
use crate::prelude::*;
use super::build_env_name;
use clap::Args;

#[derive(Args, Getters, Default)]
pub struct EditCommonArgs {
    #[arg(long, env = build_env_name!(EDITOR))]
    #[getset(get = "pub")]
    /// Name of the editor to use without any arguments. Must findable via $PATH.
    editor: Option<String>,
}

impl EditCommonArgs {
    /// Returns editor to use for opening/changing journals.
    /// It returns editor from either a CLI argument, environment variable, configuration file.
    /// If none of these sources provide an editor then the default editor of app is provided.
    pub fn resolve_editor(&self, config: &AppOptions) -> AppResult<String> {
        let borrowed = if let Some(to_use) = self.editor() {
            info!("Editor {} is used provided as cli argument.", to_use);
            to_use.as_str()
        } else {
            let config = config
                .load_config()
                .context("Error in reading config file for retrieving editor to use")?;

            match config {
                Some(editor_from_config) => match editor_from_config.editor() {
                    Some(editor) => {
                        info!("Editor {} is used provided as config file.", editor);
                        editor
                    }
                    None => return_default_editor(),
                },
                None => return_default_editor(),
            }
        };

        return Ok(borrowed.to_owned());

        fn return_default_editor() -> &'static str {
            info!("Using default editor {} .", DEFAUTL_EDITOR);
            DEFAUTL_EDITOR
        }
    }
}
