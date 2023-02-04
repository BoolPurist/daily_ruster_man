use super::build_env_name;
use clap::Args;

#[derive(Args, Getters, Default)]
pub struct EditCommonArgs {
    #[arg(long, env = build_env_name!(EDITOR))]
    #[getset(get = "pub")]
    /// Name of the editor to use without any arguments. Must findable via $PATH.
    editor: Option<String>,
}
