use super::*;

use std::borrow::Cow;
use std::str::FromStr;

use crate::core::app_config::AppConfig;
use crate::core::constants::*;
use chrono::prelude::*;
use thiserror::Error;

use crate::prelude::*;
use crate::core::date_models::units_validated::{
    ValidatedDate, ValidatedYear, ValidatedDay, ValidatedMonth,
};
#[derive(Debug, PartialEq, Eq, Getters, CopyGetters)]
pub struct DailyName {
    #[getset(get_copy = "pub")]
    date: ValidatedDate,
    name: String,
}

impl DateNameForFile for DailyName {
    fn name(&self) -> &str {
        &self.name
    }
}

impl DailyName {
    pub fn new(year: u32, month: u32, day: u32, ext: &str) -> AppResult<Self> {
        let year: ValidatedYear = year.try_into()?;
        let month: ValidatedMonth = month.try_into()?;
        let day: ValidatedDay = day.try_into()?;
        let name = Self::to_format(year.into(), month.into(), day.into(), ext);
        let date: ValidatedDate = ValidatedDate::new(year, month, day)?;

        Ok(Self { name, date })
    }

    pub fn is_in_day(&self, day: u32) -> bool {
        self.date.day() == day
    }

    fn create_name_from_date(date: NaiveDate, ext: &str) -> String {
        Self::to_format(date.year() as u32, date.month(), date.day(), ext)
    }

    fn to_format(year: u32, month: u32, day: u32, ext: &str) -> String {
        format!("{year}{DIGIT_SEP}{month:02}{DIGIT_SEP}{day:02}{DIGIT_SEP}{DAILY_INFIX}.{ext}",)
    }
}

impl ResolvePlaceholders for DailyName {
    fn resolve_variable<'a>(&self, to_resolve: &'a str) -> Cow<'a, str> {
        match to_resolve {
            super::DAY_VAR_NAME => Cow::Owned(self.date.day().to_string()),
            super::MONTH_VAR_NAME => Cow::Owned(self.date.month().to_string()),
            super::YEAR_VAR_NAME => Cow::Owned(self.date.year().to_string()),
            _ => Cow::Borrowed(to_resolve),
        }
    }
}

impl InitialabeFromTemplate for DailyName {
    fn choose_template(&self, to_choose_from: &AppConfig) -> PatchFromConfig {
        to_choose_from.daily_template()
    }
}

impl ToDateTuple for DailyName {
    fn to_date_tuple(&self) -> String {
        let date: NaiveDate = self.date.into();
        format!("{0} {1:02} {2:02}", date.year(), date.month(), date.day(),)
    }
}

impl HasYear for DailyName {
    fn year(&self) -> u32 {
        self.date.year()
    }
}
impl HasMonth for DailyName {
    fn month(&self) -> u32 {
        self.date.month()
    }
}

impl PartialOrd for DailyName {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.date.partial_cmp(&other.date)
    }
}

impl Ord for DailyName {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.date.cmp(&other.date)
    }
}

impl FromStr for DailyName {
    type Err = ParseDailyNameError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.trim().split(DIGIT_SEP);

        let (y_parsed, m_parsed, d_parsed) = match (splits.next(), splits.next(), splits.next()) {
            (Some(year), Some(month), Some(day)) => {
                let parsed_year: u32 = year.parse().or(Err(ParseDailyNameError::YearNotANumber))?;
                let parsed_month: u32 = month
                    .parse()
                    .or(Err(ParseDailyNameError::MonthNotANumber))?;
                let parsed_daiy: u32 = day.parse().or(Err(ParseDailyNameError::DayNotANumber))?;

                Ok((parsed_year, parsed_month, parsed_daiy))
            }
            (Some(_), Some(_), None) => Err(ParseDailyNameError::MissingMonth),
            (Some(_), None, None) => Err(ParseDailyNameError::MissingYear),
            _ => unreachable!(),
        }?;
        let validated = Self::new(y_parsed, m_parsed, d_parsed, MD_EXT)
            .map_err(|_| ParseDailyNameError::InvalidDate)?;

        Ok(validated)
    }
}

impl From<ValidatedDate> for DailyName {
    fn from(value: ValidatedDate) -> Self {
        let name = Self::create_name_from_date(value.into(), MD_EXT);
        Self { date: value, name }
    }
}

impl From<NaiveDate> for DailyName {
    fn from(value: NaiveDate) -> Self {
        let name = Self::create_name_from_date(value, MD_EXT);
        Self {
            date: value.into(),
            name,
        }
    }
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ParseDailyNameError {
    #[error("Year is not a number")]
    YearNotANumber,
    #[error("Month is not a number")]
    MonthNotANumber,
    #[error("Day is not a number")]
    DayNotANumber,
    #[error("No year provided for the name of daily")]
    MissingYear,
    #[error("No month provided for the name of daily")]
    MissingMonth,
    #[error("Year, month and day form a invalid date")]
    InvalidDate,
}
