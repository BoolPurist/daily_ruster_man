use chrono::{Local, Datelike};

use crate::{prelude::*, core::date_models::MonthInYear};
use super::DIGIT_SEP;
use crate::core::{constants::MD_EXT, date_models};
const PREFIX_MONTH: &str = "monthly";
#[derive(Getters, CopyGetters)]
pub struct MonthlyName {
    #[getset(get = "pub")]
    name: String,
}

impl MonthlyName {
    pub fn new(year: u32, month: u32, ext: &str) -> AppResult<Self> {
        let name = Self::create_name(year, month, ext);

        date_models::check_for_month(month)?;
        Ok(Self { name })
    }

    pub fn from_month_in_year(month_in_year: &MonthInYear) -> AppResult<Self> {
        match month_in_year {
            MonthInYear::CurrentMonth => {
                let now = Local::now().date_naive();
                Self::new(now.year() as u32, now.month(), MD_EXT)
            }
            MonthInYear::InCurrentYear(month) => {
                let now = Local::now().date_naive();
                Self::new(now.year() as u32, *month, MD_EXT)
            }
            MonthInYear::WithYear { month, year } => Self::new(*year, *month, MD_EXT),
        }
    }

    fn create_name(year: u32, month: u32, ext: &str) -> String {
        format!("{year:04}{DIGIT_SEP}{month:02}{DIGIT_SEP}{PREFIX_MONTH}.{ext}")
    }
}

#[cfg(test)]
mod testing {
    use super::*;
    #[test]
    fn should_produce_name_with_year_month() {
        assert_if_name_with_month_year(2000, 8, &format!("2000_08_{PREFIX_MONTH}.{MD_EXT}"));
        assert_if_name_with_month_year(1990, 11, &format!("1990_11_{PREFIX_MONTH}.{MD_EXT}"));
        assert_if_name_with_month_year(800, 1, &format!("0800_01_{PREFIX_MONTH}.{MD_EXT}"));
    }

    fn assert_if_name_with_month_year(y: u32, m: u32, expected: &str) {
        let given = MonthlyName::new(y, m, MD_EXT).expect("Invalid month name");

        let actual = given.name();
        assert_eq!(expected, actual);
    }
}
