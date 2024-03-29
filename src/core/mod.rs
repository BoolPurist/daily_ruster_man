pub mod app_config;
pub mod app_options;
pub mod date_models;
pub mod delete_actions;
pub mod list_queries;
pub mod open_actions;
pub mod process_handling;

mod date_filtering;
mod dates_names;
mod file_access;
mod template;

use self::dates_names::daily_names::DailyName;

pub mod constants {

    use crate::prelude::*;
    use std::path::PathBuf;
    use chrono::Datelike;

    pub const MD_EXT: &str = "md";
    pub const DAILY_INFIX: &str = "daily";
    pub const MONTHLY_LABEL_IN_NAME: &str = "monthly";
    pub const YEARLY_LABEL_IN_NAME: &str = "yearly";
    pub const DIGIT_SEP: &str = "_";
    pub const SIGN_FOR_FROM_CONF_FOLDER: char = '+';

    pub const MONTH_LOWER_BOUND: u32 = 1;
    pub const MONTH_UPPER_BOUND: u32 = 12;

    pub const DAY_LOWER_BOUND: u32 = 1;
    pub const DAY_UPPER_BOUND: u32 = 31;

    pub const ENV_PREFIX: &str = "JOURNAL_RUSTER";
    pub const CONF_FILE_NAME: &str = "config.toml";

    /// Placeholder value for which a journal inserts its day.
    pub const DAY_VAR_NAME: &str = "DAY_JOURNAL";
    /// Placeholder value for which a journal inserts its month.
    pub const MONTH_VAR_NAME: &str = "MONTH_JOURNAL";
    /// Placeholder value for which a journal inserts its year.
    pub const YEAR_VAR_NAME: &str = "YEAR_JOURNAL";

    /// Marks start of an builtin value for placeholder
    /// Example: {{ is prefix for  builtin var {{SOME_BUILTIN_VAR}}
    /// Makes sure user can still use the name of builtin variable
    pub const PREFIX_FOR_BUITLIN_VAR: &str = "{{";
    /// Marks end of an builtin value for placeholder
    /// Example: }} is prefix for  builtin var {{SOME_BUILTIN_VAR}}
    /// Makes sure user can still use the name of builtin variable
    pub const SUFFIX_FOR_BUITLIN_VAR: &str = "}}";

    const DEV_DATA_INFIX: &str = ".dev_data";

    pub static YEAR_UPPER_BOUND: SyncLazy<u32> =
        SyncLazy::new(|| chrono::NaiveDate::MAX.year() as u32);
    pub static DEV_DATA_FOLDER: SyncLazy<PathBuf> =
        SyncLazy::new(|| PathBuf::from(DEV_DATA_INFIX).join("share"));
    pub static DEV_CONF_FOLDER: SyncLazy<PathBuf> =
        SyncLazy::new(|| PathBuf::from(DEV_DATA_INFIX).join("conf"));
}
