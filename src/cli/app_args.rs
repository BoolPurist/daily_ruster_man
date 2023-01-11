pub use crate::cli::{
    edit_command::{EditByDate, EditCommand},
    month_edit_command::EditByMonthCommand,
    list_command::ListCommand,
    month_list_command::ListByMonthCommand,
};
pub use crate::core::date_models;
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
    /// Opens or creates a journal for given month in a year
    /// If given no month and year then the current month is created/opened.
    #[command(visible_alias = "me")]
    MonthEdit(EditByMonthCommand),
    /// List month journals in a given year. If not further arguments given, all months are shown.
    #[command(visible_alias = "ml")]
    MonthList(ListByMonthCommand),
}
