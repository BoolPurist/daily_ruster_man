use super::data_models::*;
use chrono::prelude::*;
use std::fmt::Display;
use std::str::FromStr;
use thiserror::Error;

const DAILY_INFIX: &str = "daily";
const DIGIT_SEP: &str = "_";
pub const MD_EXT: &str = "md";

#[derive(Debug, PartialEq, Eq)]
pub struct DailyName {
    date: NaiveDate,
    name: String,
}

impl DailyName {
    pub fn new(date: NaiveDate, ext: &str) -> Self {
        let year = date.year();
        let month = date.month();
        let day = date.day();
        let name = Self::to_format(year, month, day, ext);

        Self { name, date }
    }

    fn to_format(year: i32, month: u32, day: u32, ext: &str) -> String {
        format!("{year}{DIGIT_SEP}{month:02}{DIGIT_SEP}{day:02}{DIGIT_SEP}{DAILY_INFIX}.{ext}",)
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_date(&self) -> NaiveDate {
        self.date
    }
    pub fn create_daily_name_from(date: NaiveDate) -> Self {
        Self::new(date, MD_EXT)
    }
    pub fn create_today_name() -> DailyName {
        let now = chrono::Local::now();
        let date_now = now.date_naive();

        Self::create_daily_name_from(date_now)
    }

    fn create_from_point_and_range(range: &PastFuture, mut wanted_date: NaiveDate) -> Self {
        match range {
            PastFuture::Past(days) => wanted_date -= chrono::Duration::days(*days as i64),
            PastFuture::Future(days) => wanted_date += chrono::Duration::days(*days as i64),
        };

        Self::create_daily_name_from(wanted_date)
    }

    pub fn create_from_range(range: &PastFuture) -> Self {
        let wanted_date = chrono::Local::now().date_naive();
        Self::create_from_point_and_range(range, wanted_date)
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

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn test_parse_error() {
        assert_if_parse_fails_with("daily.md", ParseDailyNameError::MissingYear);
        assert_if_parse_fails_with("2000_daily.md", ParseDailyNameError::MissingMonth);
        assert_if_parse_fails_with("aa_02_07_daily.md", ParseDailyNameError::YearInvalid);
        assert_if_parse_fails_with("2000_z_2_daily.md", ParseDailyNameError::MonthInvalid);
        assert_if_parse_fails_with("2000_2_-7_daily.md", ParseDailyNameError::DayInvalid);
        assert_if_parse_fails_with("2000_2_32_daily.md", ParseDailyNameError::InvalidDate);
        assert_if_parse_fails_with("2000_0_1_daily.md", ParseDailyNameError::InvalidDate);
        assert_if_parse_fails_with("-78_2_2_daily.md", ParseDailyNameError::YearInvalid);
    }

    #[test]
    fn test_parse() {
        assert_parse(
            "2023_01_07_daily.md",
            NaiveDate::from_ymd_opt(2023, 1, 7).unwrap(),
        );
        assert_parse(
            "1981_11_24_daily.md",
            NaiveDate::from_ymd_opt(1981, 11, 24).unwrap(),
        );
        assert_parse(
            "2001_02_20_daily.md",
            NaiveDate::from_ymd_opt(2001, 2, 20).unwrap(),
        );
    }

    #[test]
    fn test_create_daily_in_past() {
        assert_daily_from_range(PastFuture::Past(2), 2022, 2, 8, 2022, 2, 6);
        assert_daily_from_range(PastFuture::Past(40), 2022, 2, 7, 2021, 12, 29);
        assert_daily_from_range(PastFuture::Past(0), 1980, 1, 1, 1980, 1, 1);
    }
    #[test]
    fn test_create_daily_in_future() {
        assert_daily_from_range(PastFuture::Future(2), 2022, 2, 8, 2022, 2, 10);
        assert_daily_from_range(PastFuture::Future(40), 2022, 2, 7, 2022, 3, 19);
        assert_daily_from_range(PastFuture::Future(0), 1980, 1, 1, 1980, 1, 1);
    }

    #[test]
    fn test_new() {
        assert_daily_name_new(2000, 8, 20);
        assert_daily_name_new(1988, 5, 2);
        assert_daily_name_new(2022, 1, 30);
        assert_daily_name_new(1970, 12, 1);
    }

    fn assert_daily_from_range(
        range: PastFuture,
        g_year: u32,
        g_month: u32,
        g_day: u32,
        ex_year: u32,
        ex_month: u32,
        ex_day: u32,
    ) {
        let given_date =
            NaiveDate::from_ymd_opt(g_year as i32, g_month, g_day).expect("Invalid given date");
        let actual = DailyName::create_from_point_and_range(&range, given_date);
        let expected = DailyName::new(
            NaiveDate::from_ymd_opt(ex_year as i32, ex_month, ex_day)
                .expect("Invalid expected date"),
            MD_EXT,
        );
        assert_eq!(expected, actual);
    }

    fn assert_daily_name_new(y: i32, m: u32, d: u32) {
        let given = NaiveDate::from_ymd_opt(y, m, d).expect("actual date as test input is invalid");

        let expected = format!("{y}_{m:02}_{d:02}_daily.{MD_EXT}");

        let actual = DailyName::new(given, MD_EXT);
        assert_eq!(expected, actual.get_name());
    }

    fn assert_if_parse_fails_with(invalid_input: &str, expected_error: ParseDailyNameError) {
        match invalid_input.parse::<DailyName>() {
            Ok(_) => panic!("Should not parse on this invalid input {}", invalid_input),
            Err(error) => assert_eq!(error, expected_error),
        };
    }

    fn assert_parse(valid: &str, expected: NaiveDate) {
        if let Ok(parsed) = valid.parse::<DailyName>() {
            assert_eq!(parsed.date, expected);
        } else {
            panic!("Did parse valid input correctly, ({})", valid);
        }
    }
}
