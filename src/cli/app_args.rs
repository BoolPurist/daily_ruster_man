use clap::Subcommand;
pub use clap::{Parser, Args};

pub use crate::core::date_models;
pub use crate::cli::{
    edit_command::EditCommand, month_edit_command::EditByMonthCommand, list_command::ListCommand,
    month_list_command::ListByMonthCommand,
};

#[derive(Parser, Getters)]
#[command(author, version = "0.5.1", about)]
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
    #[command(visible_alias = "d")]
    /// Deletes selected day if created.
    Delete(EditCommand),
    #[command(visible_alias = "me")]
    /// Opens or creates an entry for given month in a year.
    /// If given no month and year then the current month is created or opened.
    MonthEdit(EditByMonthCommand),
    #[command(visible_alias = "md")]
    /// Deletes selected month if it is created.
    DeleteMonth(EditByMonthCommand),
    #[command(visible_alias = "ml")]
    /// List months of a given year. If not further arguments are given, all months are shown.
    MonthList(ListByMonthCommand),
    #[command(visible_alias = "ye")]
    /// Opens or creates given entry for a year.
    YearEdit { year: Option<u32> },
    #[command(visible_alias = "yd")]
    /// Deletes selected year if created.
    DeleteYear { year: Option<u32> },
    #[command(visible_alias = "yl")]
    /// List all created entries for a year.
    YearList,
}

#[derive(Args, CopyGetters, Clone)]
pub struct DebugArgs {
    #[getset(get_copy = "pub")]
    #[arg(short, long)]
    /// If true then the journal files and config files are loaded and saved from the users folders
    /// instead of the throw away dev data folders in the project root.
    user_local_share_data: bool,
    #[getset(get_copy = "pub")]
    #[arg(short, long)]
    /// If true then selected journals will not be opened by editor nor created if not present
    /// already.
    run_editor_dry: bool,
}

macro_rules! build_env_name {
    ($field:ident) => {{
        concat!("RUSTER_JOURNAL", concat!("_", stringify!($field)))
    }};
}
#[derive(Args, CopyGetters, Getters, Clone, Default)]
pub struct GenerellArgs {
    #[getset(get_copy = "pub")]
    #[arg(short, long)]
    /// If true, then the backtrace for errors are active and even debug logs are shown
    debug: bool,
    #[arg(long, env = build_env_name!(CONFIG_PATH))]
    #[getset(get = "pub")]
    /// Uses instead provided path to find config.toml file as configuration
    config_path: Option<String>,
    #[arg(long, env = build_env_name!(DATA_PATH))]
    #[getset(get = "pub")]
    /// Uses instead provided path to read/write journal files
    data_path: Option<String>,
}
