pub mod app_config;
pub mod app_options;
pub mod date_models;
pub mod list_queries;
pub mod open_actions;

mod date_filtering;
mod dates_names;
mod file_access;
mod process_handling;
mod template;

use self::dates_names::daily_names::DailyName;

mod constants {

    use crate::prelude::*;
    use std::path::PathBuf;
    use chrono::Datelike;

    pub const MD_EXT: &str = "md";
    pub const DAILY_INFIX: &str = "daily";
    pub const MONTHLY_LABEL_IN_NAME: &str = "monthly";
    pub const YEARLY_LABEL_IN_NAME: &str = "yearly";
    pub const NVIM: &str = "nvim";
    pub const DIGIT_SEP: &str = "_";

    pub const MONTH_LOWER_BOUND: u32 = 1;
    pub const MONTH_UPPER_BOUND: u32 = 12;

    pub const DAY_LOWER_BOUND: u32 = 1;
    pub const DAY_UPPER_BOUND: u32 = 31;

    pub const CONF_FILE_NAME: &str = "config.toml";

    const DEV_DATA_INFIX: &str = ".dev_data";

    pub static YEAR_UPPER_BOUND: SyncLazy<u32> =
        SyncLazy::new(|| chrono::NaiveDate::MAX.year() as u32);
    pub static DEV_DATA_FOLDER: SyncLazy<PathBuf> =
        SyncLazy::new(|| PathBuf::from(DEV_DATA_INFIX).join("share"));
    pub static DEV_CONF_FOLDER: SyncLazy<PathBuf> =
        SyncLazy::new(|| PathBuf::from(DEV_DATA_INFIX).join("conf"));
}
