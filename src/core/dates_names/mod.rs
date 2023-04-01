pub mod daily_names;
pub mod monthly_name;
pub mod yearly_name;
pub use daily_names::DailyName;
pub use monthly_name::MonthlyName;
pub use crate::core::app_config::PatchFromConfig;

#[cfg(test)]
pub mod test_daily_names;

use crate::core::app_config::AppConfig;
use crate::core::constants::{DAY_VAR_NAME, MONTH_VAR_NAME, YEAR_VAR_NAME};

use std::{str::FromStr, borrow::Cow};

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
    fn choose_template(&self, app_options: &AppConfig) -> PatchFromConfig;
}

pub trait ResolvePlaceholders {
    fn resolve_variable<'a>(&self, to_resolve: &'a str) -> Cow<'a, str>;
}
