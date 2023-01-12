use crate::prelude::*;
use chrono::NaiveDate;
use crate::core::constants::{
    YEAR_UPPER_BOUND, DAY_LOWER_BOUND, DAY_UPPER_BOUND, MONTH_LOWER_BOUND, MONTH_UPPER_BOUND,
};

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Display, Clone, Copy)]
pub struct ValidatedYear(u32);

impl From<ValidatedYear> for u32 {
    fn from(value: ValidatedYear) -> Self {
        value.0
    }
}
impl TryFrom<u32> for ValidatedYear {
    type Error = AppError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value > *YEAR_UPPER_BOUND {
            bail!("Year must not be higher than {}", *YEAR_UPPER_BOUND)
        }

        Ok(Self(value))
    }
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Display, Clone, Copy)]
pub struct ValidatedMonth(u32);

impl From<ValidatedMonth> for u32 {
    fn from(value: ValidatedMonth) -> Self {
        value.0
    }
}
impl TryFrom<u32> for ValidatedMonth {
    type Error = AppError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if !(MONTH_LOWER_BOUND..=MONTH_UPPER_BOUND).contains(&value) {
            bail!(
                "Month must be between {} and {}.",
                MONTH_LOWER_BOUND,
                MONTH_UPPER_BOUND
            )
        }

        Ok(Self(value))
    }
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Display, Clone, Copy)]
pub struct ValidatedDay(u32);

impl From<ValidatedDay> for u32 {
    fn from(value: ValidatedDay) -> Self {
        value.0
    }
}
impl TryFrom<u32> for ValidatedDay {
    type Error = AppError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if !(DAY_LOWER_BOUND..=DAY_UPPER_BOUND).contains(&value) {
            bail!(
                "Day of month must be between {} and {}.",
                DAY_LOWER_BOUND,
                DAY_UPPER_BOUND
            )
        }

        Ok(Self(value))
    }
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Display, Clone, Copy)]
pub struct ValidatedDate(NaiveDate);

impl ValidatedDate {
    pub fn new(year: ValidatedYear, month: ValidatedMonth, day: ValidatedDay) -> AppResult<Self> {
        let year_u32: u32 = year.into();
        let date = NaiveDate::from_ymd_opt(year_u32 as i32, month.into(), day.into()).ok_or_else(
            || {
                anyhow!(
                    "Year: {}, month: {} and given day: {} together do not form a valid date",
                    year,
                    month,
                    day,
                )
            },
        )?;

        Ok(Self(date))
    }
}

impl From<ValidatedDate> for NaiveDate {
    fn from(value: ValidatedDate) -> Self {
        value.0
    }
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn should_accept_valid_years() {
        assert_accept_valid_unit::<ValidatedYear>(1);
        assert_accept_valid_unit::<ValidatedYear>(0);
        assert_accept_valid_unit::<ValidatedYear>(2000);
        assert_accept_valid_unit::<ValidatedYear>(1991);
        assert_accept_valid_unit::<ValidatedYear>(2030);
        assert_accept_valid_unit::<ValidatedYear>(1625);
        assert_accept_valid_unit::<ValidatedYear>(*YEAR_UPPER_BOUND);
    }
    #[test]
    fn should_accept_valid_month() {
        for month_index in 1..=12 {
            assert_accept_valid_unit::<ValidatedMonth>(month_index);
        }
    }
    #[test]
    fn should_accept_valid_days() {
        for month_index in 1..=31 {
            assert_accept_valid_unit::<ValidatedDay>(month_index);
        }
    }

    #[test]
    fn should_deny_invalid_year() {
        assert_deny_invalid_unit::<ValidatedYear>(*YEAR_UPPER_BOUND + 1);
        assert_deny_invalid_unit::<ValidatedYear>(*YEAR_UPPER_BOUND + 1000);
        assert_deny_invalid_unit::<ValidatedYear>(u32::MAX);
        assert_deny_invalid_unit::<ValidatedYear>(u32::MAX - 100);
    }
    #[test]
    fn should_deny_invalid_months() {
        assert_deny_invalid_unit::<ValidatedMonth>(0);
        assert_deny_invalid_unit::<ValidatedMonth>(13);
        assert_deny_invalid_unit::<ValidatedMonth>(u32::MAX);
        assert_deny_invalid_unit::<ValidatedMonth>(u32::MAX / 2);
    }
    #[test]
    fn should_deny_invalid_days() {
        assert_deny_invalid_unit::<ValidatedDay>(0);
        assert_deny_invalid_unit::<ValidatedDay>(32);
        assert_deny_invalid_unit::<ValidatedDay>(u32::MAX);
        assert_deny_invalid_unit::<ValidatedDay>(u32::MAX / 2);
    }
    #[test]
    fn should_accept_valid_dates() {
        assert_if_valid_date_is_accepted(2015, 3, 14);
        assert_if_valid_date_is_accepted(2015, 1, 14);
        assert_if_valid_date_is_accepted(4, 2, 27);
        assert_if_valid_date_is_accepted(2023, 1, 12);
        assert_if_valid_date_is_accepted(2023, 2, 28);
        assert_if_valid_date_is_accepted(2020, 2, 29);
    }
    #[test]
    fn should_deny_invalid_date() {
        assert_deny_of_invalid_dates(2015, 4, 31);
        assert_deny_of_invalid_dates(2023, 2, 29);
        assert_deny_of_invalid_dates(2020, 2, 30);
    }

    fn assert_deny_invalid_unit<T>(given: u32)
    where
        T: TryFrom<u32>,
    {
        let actual: Result<T, _> = given.try_into();
        assert!(actual.is_err());
    }
    fn assert_accept_valid_unit<T>(given: u32)
    where
        T: TryFrom<u32> + Into<u32>,
        <T as TryFrom<u32>>::Error: std::fmt::Debug,
    {
        let actual: T = given.try_into().expect("Valid year is treated as invalid");
        let actual_num: u32 = actual.into();
        assert_eq!(actual_num, given);
    }

    fn assert_if_valid_date_is_accepted(year: u32, month: u32, day: u32) {
        let (validated_year, validated_month, validated_day) =
            create_validated_ymd(year, month, day);

        match ValidatedDate::new(validated_year, validated_month, validated_day) {
            Ok(date) => {
                let actual_date: NaiveDate = date.into();
                assert_eq!(
                    actual_date,
                    NaiveDate::from_ymd_opt(year as i32, month, day).expect("")
                )
            }
            Err(error) => panic!("Error: Encountered for valid date. \n{}", error),
        }
    }

    fn assert_deny_of_invalid_dates(year: u32, month: u32, day: u32) {
        let (validated_year, validated_month, validated_day) =
            create_validated_ymd(year, month, day);
        let actual = ValidatedDate::new(validated_year, validated_month, validated_day);
        assert!(actual.is_err());
    }

    fn create_validated_ymd(
        year: u32,
        month: u32,
        day: u32,
    ) -> (ValidatedYear, ValidatedMonth, ValidatedDay) {
        let validated_year: ValidatedYear = year
            .try_into()
            .expect("day for date is not valid in general");
        let validated_month: ValidatedMonth = month
            .try_into()
            .expect("day for date is not valid in general");
        let validated_day: ValidatedDay = day
            .try_into()
            .expect("day for date is not valid in general");

        (validated_year, validated_month, validated_day)
    }
}
