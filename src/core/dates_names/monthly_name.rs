use std::str::FromStr;

use chrono::{Local, Datelike};

use crate::{
    prelude::*,
    core::{
        constants::*,
        date_models::{self, open_by::OpenByMonthInYear},
    },
};

use super::{HasYear, HasMonth, ToDateTuple};

#[derive(Getters, CopyGetters, PartialEq, Eq, Debug)]
pub struct MonthlyName {
    #[getset(get = "pub")]
    name: String,
    year: u32,
    month: u32,
}

impl MonthlyName {
    pub fn new(year: u32, month: u32, ext: &str) -> AppResult<Self> {
        let name = Self::create_name(year, month, ext);

        date_models::check_for_month(month)?;
        Ok(Self { name, year, month })
    }
    fn with_name(year: u32, month: u32, name: &str) -> AppResult<Self> {
        date_models::check_for_month(month)?;
        Ok(Self {
            name: name.to_owned(),
            year,
            month,
        })
    }

    pub fn from_month_in_year(month_in_year: &OpenByMonthInYear) -> AppResult<Self> {
        match month_in_year {
            OpenByMonthInYear::CurrentMonth => {
                let now = Local::now().date_naive();
                Self::new(now.year() as u32, now.month(), MD_EXT)
            }
            OpenByMonthInYear::InCurrentYear(month) => {
                let now = Local::now().date_naive();
                Self::new(now.year() as u32, *month, MD_EXT)
            }
            OpenByMonthInYear::WithYear { month, year } => Self::new(*year, *month, MD_EXT),
        }
    }

    fn create_name(year: u32, month: u32, ext: &str) -> String {
        format!(
            "{year:04}{0}{month:02}{0}{1}.{ext}",
            DIGIT_SEP, MONTHLY_LABEL_IN_NAME
        )
    }
}

impl ToDateTuple for MonthlyName {
    fn to_date_tuple(&self) -> String {
        format!("{:04} {:02}", self.year, self.month)
    }
}

impl PartialOrd for MonthlyName {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.month.partial_cmp(&other.month)
    }
}

impl Ord for MonthlyName {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.month.cmp(&other.month)
    }
}

impl HasYear for MonthlyName {
    fn year(&self) -> u32 {
        self.year
    }
}

impl HasMonth for MonthlyName {
    fn month(&self) -> u32 {
        self.month
    }
}

impl FromStr for MonthlyName {
    type Err = AppError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split(DIGIT_SEP);

        match (splits.next(), splits.next(), splits.next(), splits.next()) {
            (Some(year), Some(month), Some(_), None) => {
                let parsed_year: u32 = year
                    .parse()
                    .map_err(AppError::new)
                    .context("Year not parseable")?;

                let parsed_month: u32 = month
                    .parse()
                    .map_err(AppError::new)
                    .context("Month not parseable")?;

                Self::with_name(parsed_year, parsed_month, s)
            }
            _ => bail!("Invalid format"),
        }
    }
}

#[cfg(test)]
mod testing {
    use super::*;
    #[test]
    fn should_produce_name_with_year_month() {
        assert_if_name_with_month_year(
            2000,
            8,
            &format!("2000_08_{0}.{MD_EXT}", MONTHLY_LABEL_IN_NAME),
        );
        assert_if_name_with_month_year(
            1990,
            11,
            &format!("1990_11_{0}.{MD_EXT}", MONTHLY_LABEL_IN_NAME),
        );
        assert_if_name_with_month_year(
            800,
            1,
            &format!("0800_01_{0}.{MD_EXT}", MONTHLY_LABEL_IN_NAME),
        );
    }
    #[test]
    fn should_parse_from_str() {
        let given = "2000_08_monthly.md";
        let actual: MonthlyName = given
            .parse()
            .expect("Parsing should not fail in this test.");
        let expected = MonthlyName::new(2000, 8, MD_EXT).expect("Invalid month given.");

        assert_eq!(expected, actual);
    }
    #[test]
    fn should_fail_parse_str() {
        let given = "2000_08_12_monthly.md";
        let has_failed = given.parse::<MonthlyName>().is_err();

        assert!(has_failed);
    }

    #[test]
    fn should_return_month_year_pair() {
        let given = MonthlyName::new(2000, 11, MD_EXT).expect("Invalid month given.");
        let actual = given.to_date_tuple();
        let expected = "2000 11";

        assert_eq!(expected, actual);
    }

    fn assert_if_name_with_month_year(y: u32, m: u32, expected: &str) {
        let given = MonthlyName::new(y, m, MD_EXT).expect("Invalid month name");

        let actual = given.name();
        assert_eq!(expected, actual);
    }
}
