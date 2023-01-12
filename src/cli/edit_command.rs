use crate::core::date_models::open_by::{OpenByDaysInTime, OpenByDayOfYear, OpenByDayMonthYear};
use crate::AppResult;
use clap::Parser;

#[derive(Parser)]
pub struct EditCommand {
    /// If given a single argument then it opens/creates the nth journal in the past, negative value,
    /// or in the future, positive value. If not given as single argument then it represents the
    /// year of a given date.
    #[arg(allow_negative_numbers = true)]
    range_or_year: Option<i32>,
    /// if given without the third argument then it represents the ordinal day of year between 1
    /// and 366. If given with 3. argument then this argument serves as the month of given date
    /// between 1 and 12.
    day_of_year_or_month: Option<u32>,
    /// if given it serves as the day of month between 1 and 30 or 31 depenging on the month. For
    /// February it is between 1 and 28 or 29 depending on the leap year.
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
                Ok(EditByDate::Range(OpenByDaysInTime::new(past_future)))
            }
            (Some(year), Some(day_of_year), None) => {
                if year < 0 {
                    Err(anyhow!("year must be positive with day of year"))
                } else {
                    Ok(EditByDate::DayOfYear(OpenByDayOfYear::new(
                        year as u32,
                        day_of_year,
                    )))
                }
            }
            (Some(year), Some(month), Some(day)) => {
                if year < 0 {
                    Err(anyhow!("year must be positive with provided day and month"))
                } else {
                    Ok(EditByDate::DayMonthYear(OpenByDayMonthYear::new(
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
    Range(OpenByDaysInTime),
    DayOfYear(OpenByDayOfYear),
    DayMonthYear(OpenByDayMonthYear),
}
