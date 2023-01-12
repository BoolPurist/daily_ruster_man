use crate::prelude::*;
use std::str::FromStr;
use thiserror::Error;

use crate::core::constants::{DIGIT_SEP, MD_EXT, YEARLY_LABEL_IN_NAME};
use super::{HasYear, ToDateTuple, DateNameForFile};
use crate::core::date_models::units_validated::ValidatedYear;

#[derive(Debug, PartialEq, Eq)]
pub struct YearlyName {
    name: String,
    year: ValidatedYear,
}

impl DateNameForFile for YearlyName {
    fn name(&self) -> &str {
        &self.name
    }
}

impl YearlyName {
    pub fn new(year: ValidatedYear) -> Self {
        let name = Self::create_name(&year);
        Self { year, name }
    }
    fn with_name(year: ValidatedYear, name: &str) -> Self {
        let name = name.to_owned();
        Self { year, name }
    }

    fn create_name(year: &ValidatedYear) -> String {
        let year: u32 = (*year).into();
        format!(
            "{0:04}{1}{2}.{3}",
            year, DIGIT_SEP, YEARLY_LABEL_IN_NAME, MD_EXT
        )
    }
}

impl HasYear for YearlyName {
    fn year(&self) -> u32 {
        self.year.into()
    }
}

impl ToDateTuple for YearlyName {
    fn to_date_tuple(&self) -> String {
        format!("{0:04}", self.year())
    }
}

impl Ord for YearlyName {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let year: u32 = self.year.into();
        let other_year: u32 = other.year.into();
        year.cmp(&other_year)
    }
}

impl PartialOrd for YearlyName {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for YearlyName {
    type Err = ParseErrorForYearName;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name = s;
        let mut splits = name.split(DIGIT_SEP);
        match (splits.next(), splits.next()) {
            (Some(year), Some(_)) => {
                let year: u32 = year
                    .parse()
                    .map_err(|_| ParseErrorForYearName::MissingYear)?;
                let year = year
                    .try_into()
                    .map_err(ParseErrorForYearName::InvalidYear)?;

                Ok(Self::with_name(year, name))
            }
            _ => Err(ParseErrorForYearName::InvalidFormat),
        }
    }
}

#[derive(Debug, Error)]
pub enum ParseErrorForYearName {
    #[error("Year field is not a valid number")]
    MissingYear,
    #[error("Valid is not in the forma of [year]_yearly.md")]
    InvalidFormat,
    #[error("{0}")]
    InvalidYear(AppError),
}

#[cfg(test)]
mod testing {
    use super::*;
    use test_case::test_case;

    #[test_case(2000 => String::from("2000_yearly.md"))]
    #[test_case(0100 => String::from("0100_yearly.md"))]
    #[test_case(28 => String::from("0028_yearly.md"))]
    #[test_case(5 => String::from("0005_yearly.md"))]
    fn should_construct_correct_name(year: u32) -> String {
        let yearly_name = create_yearly_name_from(year);
        yearly_name.name().to_owned()
    }

    fn create_yearly_name_from(year: u32) -> YearlyName {
        let validated_year: ValidatedYear = year.try_into().expect("Given year is not valid");
        YearlyName::new(validated_year)
    }

    fn validate_parsed_yearly_name(expected_year: u32) -> impl Fn(YearlyName) {
        move |actual: YearlyName| {
            assert_eq!(expected_year, actual.year());
        }
    }

    #[test_case("aa_aa.md")]
    #[test_case("222a_aa.md")]
    #[test_case("3332222_aa.md")]
    #[test_case("33")]
    fn should_fail_parsing(input: &str) {
        let daily_name: Result<YearlyName, _> = input.parse();
        assert!(daily_name.is_err())
    }

    #[test_case(2000 => String::from("2000") )]
    #[test_case(120 => String::from("0120") )]
    #[test_case(20 => String::from("0020") )]
    #[test_case(3 => String::from("0003") )]
    fn should_produce_date_tuple(year: u32) -> String {
        let yearly_name = create_yearly_name_from(year);
        yearly_name.to_date_tuple()
    }

    #[test_case("2000_yearly.md" => using validate_parsed_yearly_name(2000))]
    #[test_case("1988_yearly.md" => using validate_parsed_yearly_name(1988))]
    #[test_case("188_yearly.md" => using validate_parsed_yearly_name(188))]
    #[test_case("1_yearly.md" => using validate_parsed_yearly_name(1))]
    #[test_case("24_yearly.md" => using validate_parsed_yearly_name(24))]
    fn should_parse(name: &str) -> YearlyName {
        name.parse().expect("Should fail for valid input")
    }
}
