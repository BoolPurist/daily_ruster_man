use clap::Subcommand;
pub use clap::{Parser, Args};

pub use crate::core::date_models;
pub use crate::cli::{
    edit_command::EditCommand,
    month_edit_command::EditByMonthCommand,
    list_command::ListCommand,
    month_list_command::ListByMonthCommand,
    deletion_arguments::{DeleteDateArg, DeleteYearArg, DeleteMonthArg},
};

use crate::cli::build_env_name;

use super::edit_command::EditCommandAndArgs;
use super::edit_year::EditByYear;
use super::month_edit_command::EditByMonthCommandAndArgs;

#[derive(Parser, Getters)]
#[command(author, version = "0.5.4", about)]
#[getset(get = "pub")]
/// Create/manage daily, monthly and yearly journals with your editor of choise.
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
    /// Opens or creates an journal for certain day with your editor of choice. Opens for today
    /// if no arguments are given.
    /// Options flags can be combined for selecting a day in past or future
    #[command(visible_alias = "e")]
    Edit(EditCommandAndArgs),
    #[command(visible_alias = "d")]
    /// Deletes the selected day if it was created.
    Delete(DeleteDateArg),
    #[command(visible_alias = "me")]
    /// Opens or creates an entry for given month in a year.
    /// If given no month and year then the current month is created or opened.
    MonthEdit(EditByMonthCommandAndArgs),
    #[command(visible_alias = "md")]
    /// Deletes selected month if it was created.
    DeleteMonth(DeleteMonthArg),
    #[command(visible_alias = "ml")]
    /// List months of a given year. If no further arguments are given, all months are shown.
    MonthList(ListByMonthCommand),
    #[command(visible_alias = "ye")]
    /// Opens or creates journal for a year.
    YearEdit(EditByYear),
    #[command(visible_alias = "yd")]
    /// Deletes selected year if it was created.
    DeleteYear(DeleteYearArg),
    #[command(visible_alias = "yl")]
    /// Lists all created journal for a year.
    YearList,
}

#[derive(Args, CopyGetters, Clone)]
pub struct DebugArgs {
    #[getset(get_copy = "pub")]
    #[arg(short, long)]
    /// If true then the journal files and config files are loaded and saved from the users folders
    /// instead of the throw away dev data folder at the project root.
    user_local_share_data: bool,
    #[getset(get_copy = "pub")]
    #[arg(short, long)]
    /// If true then the selected journals will not be opened by editor nor created if not present
    /// already.
    run_editor_dry: bool,
}

#[derive(Args, CopyGetters, Getters, Clone, Default)]
pub struct GenerellArgs {
    #[getset(get_copy = "pub")]
    #[arg(short, long)]
    /// If true, then the backtrace for errors are active and even debug logs are shown
    debug: bool,
    #[arg(long, env = build_env_name!(CONFIG_PATH))]
    #[getset(get = "pub")]
    /// Uses provided path to find config folder for this app
    config_path: Option<String>,
    #[arg(long, env = build_env_name!(DATA_PATH))]
    #[getset(get = "pub")]
    /// Uses provided path to find folder with saved journals
    data_path: Option<String>,
}
