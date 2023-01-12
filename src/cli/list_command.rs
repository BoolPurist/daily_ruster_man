use clap::Parser;
use crate::AppResult;
use crate::core::date_models::find_by::FindByYearMonthDay;

#[derive(Parser)]
pub struct ListCommand {
    /// Filter all dates by the given year. Example: if year is 2013, only daily entries within
    /// year 2013 are listed
    #[arg(short, long)]
    year: Option<u32>,
    /// Filter all dates by the given month. Example: if month is 8, only daily entries within
    /// month August are listed.
    #[arg(short, long)]
    month: Option<u32>,
    /// Filter all dates by the given day. Example: if day is 12, only daily entries within
    /// on 12th of month are listed.
    #[arg(short, long)]
    day_of_month: Option<u32>,
}

impl ListCommand {
    pub fn to_date_filter(&self) -> AppResult<FindByYearMonthDay> {
        FindByYearMonthDay::new(self.year, self.month, self.day_of_month)
    }
}
