use clap::Args;
use super::app_args::{EditByMonthCommand, EditCommand};

#[derive(Args, CopyGetters, Getters)]
pub struct DeleteDateArg {
    #[command(flatten)]
    #[getset(get = "pub")]
    date: EditCommand,
    #[command(flatten)]
    #[getset(get = "pub")]
    common_arg: CommonDeleteArg,
}
#[derive(Args, CopyGetters, Getters)]
pub struct DeleteMonthArg {
    #[command(flatten)]
    #[getset(get = "pub")]
    month: EditByMonthCommand,
    #[command(flatten)]
    #[getset(get = "pub")]
    common_arg: CommonDeleteArg,
}
#[derive(Args, CopyGetters, Getters)]
pub struct DeleteYearArg {
    /// which year journal to delete
    #[getset(get = "pub")]
    year: u32,
    #[command(flatten)]
    #[getset(get = "pub")]
    common_arg: CommonDeleteArg,
}

#[derive(Args, CopyGetters)]
pub struct CommonDeleteArg {
    #[arg(long, short, env = build_env_name!(SKIP_CONFIRMATION))]
    #[getset(get_copy = "pub")]
    /// If provided then no prompt will show up for confirmation before a deletion
    skip_confirmation: bool,
}
