pub use crate::cli::{
    edit_command::{EditByDate, EditCommand},
    list_command::ListCommand,
};
pub use crate::core::data_models;
pub use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub enum CliArgs {
    /// Shows all created daily journals so far.
    #[command(visible_alias = "l")]
    List(ListCommand),
    /// Opens or creates a journal for today or a given date via nvim
    #[command(visible_alias = "e")]
    Edit(EditCommand),
}
