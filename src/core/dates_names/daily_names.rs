use crate::core::constants::*;
use crate::core::date_models::open_by::{OpenByDaysInTime, OpenByDayOfYear, OpenByDayMonthYear};
use super::{HasYear, HasMonth, ToDateTuple};
use crate::prelude::*;
use chrono::prelude::*;

use std::fmt::Display;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Getters, CopyGetters)]
pub struct DailyName {
    #[getset(get_copy = "pub")]
    date: NaiveDate,
    #[getset(get = "pub")]
    name: String,
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
impl ToDateTuple for DailyName {
    fn to_date_tuple(&self) -> String {
        let date = self.date;
        format!("{0} {1:02} {2:02}", date.year(), date.month(), date.day(),)
    }
}

impl DailyName {
    pub fn new(date: NaiveDate, ext: &str) -> Self {
        let year = date.year();
        let month = date.month();
        let day = date.day();
        let name = Self::to_format(year, month, day, ext);

        Self { name, date }
    }

    pub fn create_from_ordinal_day(day_of_year: &OpenByDayOfYear) -> AppResult<Self> {
        let (year, ordinal_day) = (day_of_year.year(), day_of_year.day_of_year());
        let ordinal_date = NaiveDate::from_yo_opt(year as i32, ordinal_day).ok_or_else(|| {
            anyhow!(
                "Year ({}) or day of the year is ({}) not valid.{}",
                year,
                ordinal_day,
                ORDINAL_DAY_SUGGESTION
            )
        })?;
        Ok(Self::new(ordinal_date, MD_EXT))
    }
    pub fn create_from_year_month_day(year_month_day: &OpenByDayMonthYear) -> AppResult<Self> {
        let (year, month, day) = (
            year_month_day.year() as i32,
            year_month_day.month(),
            year_month_day.day(),
        );
        let ymd = NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| {
            anyhow!(
                "Year, month or day are not valid.{}",
                YEAR_MONTH_DAY_SUGGESTION
            )
        })?;
        Ok(Self::new(ymd, MD_EXT))
    }

    pub fn create_from_range(range: &OpenByDaysInTime) -> Self {
        let wanted_date = chrono::Local::now().date_naive();
        Self::create_from_point_and_range(range, wanted_date)
    }

    pub fn create_daily_name_from(date: NaiveDate) -> Self {
        Self::new(date, MD_EXT)
    }
    pub fn create_today_name() -> DailyName {
        let now = chrono::Local::now();
        let date_now = now.date_naive();

        Self::create_daily_name_from(date_now)
    }

    pub fn is_in_day(&self, day: u32) -> bool {
        self.date.day() == day
    }

    pub fn create_from_point_and_range(
        range: &OpenByDaysInTime,
        mut wanted_date: NaiveDate,
    ) -> Self {
        match range {
            OpenByDaysInTime::Past(days) => wanted_date -= chrono::Duration::days(*days as i64),
            OpenByDaysInTime::Future(days) => wanted_date += chrono::Duration::days(*days as i64),
        };

        Self::create_daily_name_from(wanted_date)
    }

    fn to_format(year: i32, month: u32, day: u32, ext: &str) -> String {
        format!("{year}{DIGIT_SEP}{month:02}{DIGIT_SEP}{day:02}{DIGIT_SEP}{DAILY_INFIX}.{ext}",)
    }
}

impl HasYear for DailyName {
    fn year(&self) -> u32 {
        self.date().year() as u32
    }
}
impl HasMonth for DailyName {
    fn month(&self) -> u32 {
        self.date().month()
    }
}

impl Display for DailyName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl FromStr for DailyName {
    type Err = ParseDailyNameError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split(DIGIT_SEP);

        let (y_parsed, m_parsed, d_parsed) = match (splits.next(), splits.next(), splits.next()) {
            (Some(year), Some(month), Some(day)) => {
                let parsed_year: i32 = year.parse().or(Err(ParseDailyNameError::YearInvalid))?;
                let parsed_month: u32 = month.parse().or(Err(ParseDailyNameError::MonthInvalid))?;
                let parsed_daiy: u32 = day.parse().or(Err(ParseDailyNameError::DayInvalid))?;

                Ok((parsed_year, parsed_month, parsed_daiy))
            }
            (Some(_), Some(_), None) => Err(ParseDailyNameError::MissingMonth),
            (Some(_), None, None) => Err(ParseDailyNameError::MissingYear),
            _ => unreachable!(),
        }?;

        if y_parsed < 0 {
            return Err(ParseDailyNameError::YearInvalid);
        }

        let date = NaiveDate::from_ymd_opt(y_parsed, m_parsed, d_parsed)
            .ok_or(ParseDailyNameError::InvalidDate)?;

        let new_self = Self {
            name: s.to_owned(),
            date,
        };
        Ok(new_self)
    }
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ParseDailyNameError {
    #[error("Invalid format for year")]
    YearInvalid,
    #[error("Invalid format for month")]
    MonthInvalid,
    #[error("Invalid format for day")]
    DayInvalid,
    #[error("No year provided for the name of daily")]
    MissingYear,
    #[error("No month provided for the name of daily")]
    MissingMonth,
    #[error("Date is not valid")]
    InvalidDate,
}