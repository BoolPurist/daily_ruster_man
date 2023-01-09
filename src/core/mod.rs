pub mod data_models;
pub mod list_queries;
pub mod open_actions;

mod daily_filtering;
mod daily_names;
mod file_access;
mod process_handling;

use self::daily_names::DailyName;

mod constants {
    pub const MD_EXT: &str = "md";
}
