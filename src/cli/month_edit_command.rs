use clap::Parser;
use crate::{
    core::date_models::{
        open_by::OpenByMonthInYear,
        units_validated::{ValidatedMonth, ValidatedYear},
    },
    AppResult,
};

#[derive(Parser)]
pub struct EditByMonthCommand {
    /// month of current or given year.
    /// If given without year then month is opene for the current year
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
