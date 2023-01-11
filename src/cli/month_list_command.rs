use crate::core::date_models::FindByMonthInYear;
use clap::Parser;
#[derive(Parser)]
pub struct ListByMonthCommand {
    /// Month to search for in year.
    month: Option<u32>,
    /// year in which months to search for
    year: Option<u32>,
}

impl ListByMonthCommand {
    pub fn create_find_month_in_year(&self) -> FindByMonthInYear {
        match (self.month, self.year) {
            (None, None) => FindByMonthInYear::All,
            (Some(month), None) => FindByMonthInYear::InCurrentYear(month),
            (Some(month), Some(year)) => FindByMonthInYear::MonthYear { month, year },
            _ => unreachable!(),
        }
    }
}
