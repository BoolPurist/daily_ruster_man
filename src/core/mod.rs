pub mod date_models;
pub mod list_queries;
pub mod open_actions;

mod date_filtering;
mod dates_names;
mod file_access;
mod process_handling;

use self::dates_names::daily_names::DailyName;

mod constants {
    pub const MD_EXT: &str = "md";
}
