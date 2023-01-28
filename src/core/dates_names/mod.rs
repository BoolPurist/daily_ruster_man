pub mod daily_names;
pub mod monthly_name;
#[cfg(test)]
pub mod test_daily_names;
pub mod yearly_name;
pub use daily_names::DailyName;
use crate::core::app_config::AppConfig;
pub use monthly_name::MonthlyName;
use std::str::FromStr;
use crate::prelude::*;

fn try_load_and_choose_template(
    on_choose_template: impl Fn(&AppConfig) -> AppResult<Option<String>>,
) -> AppResult<Option<String>> {
    let app_config = AppConfig::try_from_file_system()?;
    if let Some(config) = app_config {
        on_choose_template(&config)
    } else {
        Ok(None)
    }
}

pub trait HasYear {
    fn year(&self) -> u32;

    fn is_in_year(&self, other_year: u32) -> bool {
        self.year() == other_year
    }
}
pub trait HasMonth {
    fn month(&self) -> u32;

    fn is_in_month(&self, other_month: u32) -> bool {
        self.month() == other_month
    }
}

pub trait ToDateTuple {
    fn to_date_tuple(&self) -> String;
}

pub trait DateNameForFile: ToDateTuple + FromStr + Ord {
    fn name(&self) -> &str;
}

pub trait InitialabeFromTemplate {
    fn try_get_template(&self) -> AppResult<Option<String>>;
}
