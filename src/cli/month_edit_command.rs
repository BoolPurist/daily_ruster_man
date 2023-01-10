use clap::Parser;
use crate::core::date_models::MonthInYear;
#[derive(Parser)]
pub struct EditByMonthCommand {
    /// month of current or given year.
    /// If given without year then month is opene for the current year
    month: Option<u32>,
    /// year in which the month resides
    year: Option<u32>,
}

impl EditByMonthCommand {
    pub fn create_month_in_year(&self) -> MonthInYear {
        match (self.month, self.year) {
            (None, None) => MonthInYear::CurrentMonth,
            (Some(month), None) => MonthInYear::InCurrentYear(month),
            (Some(month), Some(year)) => MonthInYear::WithYear { month, year },
            _ => unreachable!(),
        }
    }
}
