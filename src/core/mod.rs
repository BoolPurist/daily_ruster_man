pub mod date_models;
pub mod list_queries;
pub mod open_actions;

mod date_filtering;
mod dates_names;
mod file_access;
mod process_handling;

use self::dates_names::daily_names::DailyName;

mod constants {
    use chrono::Datelike;
    pub const MD_EXT: &str = "md";
    pub const DAILY_INFIX: &str = "daily";
    pub const MONTHLY_LABEL_IN_NAME: &str = "monthly";
    pub const NVIM: &str = "nvim";
    pub const DIGIT_SEP: &str = "_";

    pub const MONTH_LOWER_BOUND: u32 = 1;
    pub const MONTH_UPPER_BOUND: u32 = 12;

    pub const DAY_LOWER_BOUND: u32 = 1;
    pub const DAY_UPPER_BOUND: u32 = 31;

    pub const ORDINAL_DAY_SUGGESTION: &str = "
Usual range for day is between 1 and 365 or 366 depending on the year and year shoud not be to big.";
    pub const YEAR_MONTH_DAY_SUGGESTION: &str = "
Year should not be too big. Month must be between 1 and 12.
Day must be between 1 and 28, 29, 30 or 31 depending on the month.";

    lazy_static! {
        pub static ref YEAR_UPPER_BOUND: u32 = chrono::NaiveDate::MAX.year() as u32;
    }
}
