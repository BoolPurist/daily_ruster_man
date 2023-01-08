pub use crate::core::data_models;
use crate::AppResult;
pub use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub enum CliArgs {
    /// Shows all created daily journals so far.
    #[command(visible_alias = "l")]
    List(ListCommand),
    /// Opens or creates a journal for today via nvim
    #[command(visible_alias = "e")]
    Edit(EditCommand),
}

#[derive(Parser)]
pub struct ListCommand;
#[derive(Parser)]
pub struct EditCommand {
    #[arg(allow_negative_numbers = true)]
    range_or_year: Option<i32>,
    day_of_year_or_month: Option<u32>,
    day_of_month: Option<u32>,
}

impl EditCommand {
    pub fn get_date_query(&self) -> AppResult<EditByDate> {
        match (
            self.range_or_year,
            self.day_of_year_or_month,
            self.day_of_month,
        ) {
            (Some(past_future), None, None) => {
                Ok(EditByDate::Range(data_models::PastFuture::new(past_future)))
            }
            (Some(year), Some(day_of_year), None) => {
                if year < 0 {
                    Err(anyhow!("year must be positive with day of year"))
                } else {
                    Ok(EditByDate::DayOfYear(data_models::DayOfYear::new(
                        year as u32,
                        day_of_year,
                    )))
                }
            }
            (Some(year), Some(month), Some(day)) => {
                if year < 0 {
                    Err(anyhow!("year must be positive with provided day and month"))
                } else {
                    Ok(EditByDate::DayMonthYear(data_models::DayMonthYear::new(
                        year as u32,
                        month,
                        day,
                    )))
                }
            }
            (None, None, None) => Ok(EditByDate::None),
            _ => unreachable!(),
        }
    }
}

pub enum EditByDate {
    None,
    Range(data_models::PastFuture),
    DayOfYear(data_models::DayOfYear),
    DayMonthYear(data_models::DayMonthYear),
}
