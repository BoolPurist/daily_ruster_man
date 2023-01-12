use crate::{
    core::date_models::{
        find_by::FindByMonthInYear,
        units_validated::{ValidatedMonth, ValidatedYear},
    },
    AppResult,
};
use clap::Parser;
#[derive(Parser)]
pub struct ListByMonthCommand {
    /// Month to search for in year.
    month: Option<u32>,
    /// year in which months to search for
    year: Option<u32>,
}

impl ListByMonthCommand {
    pub fn create_find_month_in_year(&self) -> AppResult<FindByMonthInYear> {
        match (self.month, self.year) {
            (None, None) => Ok(FindByMonthInYear::All),
            (Some(month), None) => {
                let month: ValidatedMonth = month.try_into()?;
                Ok(FindByMonthInYear::InCurrentYear(month))
            }
            (Some(month), Some(year)) => {
                let month: ValidatedMonth = month.try_into()?;
                let year: ValidatedYear = year.try_into()?;
                Ok(FindByMonthInYear::MonthYear { month, year })
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod testing {
    use super::*;
    use test_case::test_case;

    fn validate_in_current_year(expected: u32) -> impl Fn(FindByMonthInYear) {
        move |actual: FindByMonthInYear| {
            if let FindByMonthInYear::InCurrentYear(actual_month) = actual {
                let to_compare: u32 = actual_month.into();
                assert_eq!(to_compare, expected);
            } else {
                panic!(
                    "Expected variant {}",
                    stringify!(FindByMonthInYear::InCurrentYear)
                )
            }
        }
    }
    fn validate_in_month_year(
        (expected_month, expected_year): (u32, u32),
    ) -> impl Fn(FindByMonthInYear) {
        move |actual: FindByMonthInYear| {
            if let FindByMonthInYear::MonthYear { month, year } = actual {
                let month: u32 = month.into();
                let year: u32 = year.into();
                assert_eq!(expected_month, month);
                assert_eq!(expected_year, year);
            } else {
                panic!(
                    "Expected variant {}",
                    stringify!(FindByMonthInYear::MonthYear)
                )
            }
        }
    }

    #[test_case(None, None => FindByMonthInYear::All ; "Should filter nothing aka listing all months")]
    #[test_case(Some(2), None => using validate_in_current_year(2) ; "Should filter by month in the current year")]
    #[test_case(Some(4), Some(1970) => using validate_in_month_year((4, 1970)) ; "Should filter by month and year")]
    fn should_produces_correct_find_by_month_year(
        month: Option<u32>,
        year: Option<u32>,
    ) -> FindByMonthInYear {
        let given = ListByMonthCommand { month, year };
        given
            .create_find_month_in_year()
            .expect("Should produce error for valid month and year")
    }
}
