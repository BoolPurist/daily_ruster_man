pub mod conf;
mod date_filtering;
pub mod date_models;
mod dates_names;

mod file_access;
pub mod list_queries;
pub mod open_actions;
mod process_handling;

use self::dates_names::daily_names::DailyName;

mod constants {
    use std::path::{Path, PathBuf};

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

    lazy_static! {
        pub static ref YEAR_UPPER_BOUND: u32 = chrono::NaiveDate::MAX.year() as u32;
        pub static ref DEV_APP_FOLDER: PathBuf = PathBuf::from(".dev_data");
        pub static ref DEV_DATA_FOLDER: PathBuf = DEV_APP_FOLDER.join(PathBuf::from("share"));
        pub static ref DEV_CONF_FOLDER: PathBuf = DEV_APP_FOLDER.join(PathBuf::from("conf"));
    }
}
