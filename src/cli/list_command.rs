use clap::Parser;
use crate::AppResult;
use crate::core::date_models::FilterParamsYmD;

#[derive(Parser)]
pub struct ListCommand {
    /// Year of date
    #[arg(short, long)]
    year: Option<u32>,
    /// month of the year
    #[arg(short, long)]
    month: Option<u32>,
    /// day of the month
    #[arg(short, long)]
    day_of_month: Option<u32>,
}

impl ListCommand {
    pub fn create_ymd_listing(&self) -> AppResult<FilterParamsYmD> {
        FilterParamsYmD::new(self.year, self.month, self.day_of_month)
    }
}
