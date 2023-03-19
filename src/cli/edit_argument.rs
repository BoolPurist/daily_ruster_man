use crate::core::app_options::AppOptions;
use crate::prelude::*;
use super::build_env_name;
use clap::Args;

#[derive(Args, Getters, CopyGetters, Default, Setters)]
pub struct EditCommonArgs {
    #[arg(long, env = build_env_name!(EDITOR))]
    #[getset(get = "pub", set = "pub")]
    /// Name of the editor to use without any arguments. Must findable via $PATH.
    editor: Option<String>,
    #[arg(short, long)]
    #[getset(get_copy = "pub")]
    /// If given then the content of selected journal is only printed out to stdout
    /// without any involvement of editor.
    /// Output will be empty if the selected journal was not created so far.
    show_only: bool,
}

impl EditCommonArgs {
    pub const DEFAUTL_EDITOR: &str = "vim";
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
            info!("Using default editor {} .", EditCommonArgs::DEFAUTL_EDITOR);
            EditCommonArgs::DEFAUTL_EDITOR
        }
    }
}
