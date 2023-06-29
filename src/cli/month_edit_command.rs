use clap::{Parser, Args};
use date_validation_types::{ValidatedMonth, ValidatedYear};
use crate::{core::date_models::open_by::OpenByMonthInYear, AppResult};

use super::edit_argument::EditCommonArgs;

#[derive(Parser, Getters)]
#[getset(get = "pub")]
pub struct EditByMonthCommandAndArgs {
    #[command(flatten)]
    command: EditByMonthCommand,
    #[command(flatten)]
    option: EditCommonArgs,
}
#[derive(Args)]
pub struct EditByMonthCommand {
    /// month of current or given year.
    /// If given without year then month is opened for the current year
    month: Option<u32>,
    /// year in which the month resides
    year: Option<u32>,
}

impl EditByMonthCommand {
    pub fn to_valid_ym_pair(&self) -> AppResult<OpenByMonthInYear> {
        match (self.month, self.year) {
            (None, None) => Ok(OpenByMonthInYear::CurrentMonth),
            (Some(month), None) => {
                let month: ValidatedMonth = month.try_into()?;
                Ok(OpenByMonthInYear::InCurrentYear(month))
            }
            (Some(month), Some(year)) => {
                let month: ValidatedMonth = month.try_into()?;
                let year: ValidatedYear = year.try_into()?;
                Ok(OpenByMonthInYear::WithYear { month, year })
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod testing {
    use super::*;
    #[test]
    fn should_map_to_correct_exact_month_with_year() {
        const MONTH: u32 = 12;
        const YEAR: u32 = 1990;
        let month: Option<u32> = Some(MONTH);
        let year: Option<u32> = Some(YEAR);
        let given = EditByMonthCommand { month, year };

        let actual = given.to_valid_ym_pair().expect("Invalid month in year");
        assert!(matches!(
            actual,
            OpenByMonthInYear::WithYear { month, year }
            if month == MONTH.try_into().unwrap()
            && year == YEAR.try_into().unwrap()
        ));
    }

    #[test]
    fn should_map_to_month_in_current_year() {
        const MONTH: u32 = 12;
        let month: Option<u32> = Some(MONTH);
        let year: Option<u32> = None;
        let given = EditByMonthCommand { month, year };

        let actual = given.to_valid_ym_pair().expect("Invalid month in year");
        assert!(matches!(
            actual,
            OpenByMonthInYear::InCurrentYear(month)
            if month == MONTH.try_into().unwrap()
        ));
    }
    #[test]
    fn should_map_to_current_month() {
        let month: Option<u32> = None;
        let year: Option<u32> = None;
        let given = EditByMonthCommand { month, year };

        let actual = given.to_valid_ym_pair().expect("Invalid month in year");
        assert!(matches!(actual, OpenByMonthInYear::CurrentMonth));
    }
}
