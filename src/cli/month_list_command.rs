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
