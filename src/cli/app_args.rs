use clap::Subcommand;
pub use clap::{Parser, Args};

pub use crate::core::date_models;
pub use crate::cli::{
    edit_command::EditCommand, month_edit_command::EditByMonthCommand, list_command::ListCommand,
    month_list_command::ListByMonthCommand,
};

#[derive(Parser, Getters)]
#[command(author, version, about)]
#[getset(get = "pub")]
pub struct CliArgs {
    #[cfg(debug_assertions)]
    #[command(flatten)]
    debug_args: DebugArgs,
    #[command(flatten)]
    args: GenerellArgs,
    #[command(subcommand)]
    commands: AppCommands,
}

#[derive(Subcommand)]
#[command(author, version, about)]
pub enum AppCommands {
    /// Shows created daily entries so far.
    #[command(visible_alias = "l")]
    List(ListCommand),
    /// Opens or creates an entry for today or for a given date via your editor of choice.
    /// Note: Options flags can be combined for filtering. Example: --year 2013 and --month 08
    /// shows all created daily entries in year August in year 2013.
    #[command(visible_alias = "e")]
    Edit(EditCommand),
    /// Opens or creates an entry for given month in a year.
    /// If given no month and year then the current month is created or opened.
    #[command(visible_alias = "me")]
    MonthEdit(EditByMonthCommand),
    /// List months of a given year. If not further arguments are given, all months are shown.
    #[command(visible_alias = "ml")]
    MonthList(ListByMonthCommand),
    /// Opens or creates given entry for a year.
    #[command(visible_alias = "ye")]
    YearEdit { year: Option<u32> },
    /// List all created entries for a year.
    #[command(visible_alias = "yl")]
    YearList,
}

#[derive(Args, CopyGetters, Clone)]
pub struct DebugArgs {
    #[getset(get_copy = "pub")]
    #[arg(short, long)]
    user_local_share: bool,
}

#[derive(Args, CopyGetters, Clone)]
pub struct GenerellArgs {
    #[getset(get_copy = "pub")]
    #[arg(short, long)]
    debug: bool,
}
