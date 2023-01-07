pub use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub enum CliArgs {
    /// Shows all created daily journals so far.
    #[command(visible_alias = "l")]
    List(ListCommand),
    /// Opens or creates a journal for today via nvim
    #[command(visible_alias = "e")]
    Edit(EditCommand),
}

#[derive(Parser)]
pub struct ListCommand;
#[derive(Parser)]
pub struct EditCommand;
