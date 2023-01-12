use clap::Parser;
use crate::core::date_models::open_by::OpenByMonthInYear;

#[derive(Parser)]
pub struct EditByMonthCommand {
    /// month of current or given year.
    /// If given without year then month is opene for the current year
    month: Option<u32>,
    /// year in which the month resides
    year: Option<u32>,
}

impl EditByMonthCommand {
    pub fn create_month_in_year(&self) -> OpenByMonthInYear {
        match (self.month, self.year) {
            (None, None) => OpenByMonthInYear::CurrentMonth,
            (Some(month), None) => OpenByMonthInYear::InCurrentYear(month),
            (Some(month), Some(year)) => OpenByMonthInYear::WithYear { month, year },
            _ => unreachable!(),
        }
    }
}
