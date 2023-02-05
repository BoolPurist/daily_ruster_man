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
    /// If provided as the only argument then all created monthly journals of this given year are shown
    year: Option<u32>,
    /// Will list the one month of a given year.
    month: Option<u32>,
}

impl ListByMonthCommand {
    pub fn create_find_month_in_year(&self) -> AppResult<FindByMonthInYear> {
        match (self.year, self.month) {
            (None, None) => Ok(FindByMonthInYear::All),
            (Some(year), None) => {
                let year: ValidatedYear = year.try_into()?;
                Ok(FindByMonthInYear::InCurrentYear(year))
            }
            (Some(year), Some(month)) => {
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
        (expected_year, expected_month): (u32, u32),
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
    #[test_case(Some(1970), None => using validate_in_current_year(1970) ; "Should filter by month in the current year")]
    #[test_case(Some(1970), Some(2) => using validate_in_month_year((1970, 2)) ; "Should filter by month and year")]
    fn should_produces_correct_find_by_month_year(
        year: Option<u32>,
        month: Option<u32>,
    ) -> FindByMonthInYear {
        let given = ListByMonthCommand { month, year };
        given
            .create_find_month_in_year()
            .expect("Should not produce error for valid month and year")
    }
}
