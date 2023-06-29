use crate::core::date_models::open_by::OpenByDaysInTime;
use crate::AppResult;
use date_validation_types::ValidatedDate;
use chrono::Local;
use clap::{Parser, Args};
use super::edit_argument::EditCommonArgs;

#[derive(Parser, Default, Getters)]
#[getset(get = "pub")]
pub struct EditCommandAndArgs {
    #[command(flatten)]
    command: EditCommand,
    #[command(flatten)]
    option: EditCommonArgs,
}

#[derive(Args, Default)]
pub struct EditCommand {
    /// If given as a single negative argument then it opens/creates the n-times day entry in the past.
    /// If given as a single positive argument then it opens/creates the n-times day entry in the
    /// future. If more then one argument is given then this argument presents the year of a
    /// date.
    #[arg(allow_negative_numbers = true)]
    range_or_year: Option<i32>,
    /// if given without the third argument then it represents the ordinal day of year between 1
    /// and 366. If 3. arguments are given then this argument serves as the month of given date
    /// between 1 and 12.
    day_of_year_or_month: Option<u32>,
    /// if given it serves as the day of month between 1 and 30 or 31 depenging on the month. For
    /// February it is between 1 and 28 or 29 depending on the leap year.
    day_of_month: Option<u32>,
}

impl EditCommand {
    pub fn to_advance_now(&self) -> AppResult<ValidatedDate> {
        self.to_advance_valid_date(Local::now().date_naive().into())
    }

    fn to_advance_valid_date(&self, now: ValidatedDate) -> AppResult<ValidatedDate> {
        match (
            self.range_or_year,
            self.day_of_year_or_month,
            self.day_of_month,
        ) {
            (Some(past_future), None, None) => {
                let range = OpenByDaysInTime::new(past_future);
                let in_past_or_future = range.from_point_in_time(now)?;

                Ok(in_past_or_future)
            }
            (Some(year), Some(day_of_year), None) => {
                if year < 0 {
                    Err(anyhow!("year must be positive with day of year"))
                } else {
                    let ordinal_date = ValidatedDate::from_ordinal(year as u32, day_of_year)?;
                    Ok(ordinal_date)
                }
            }
            (Some(year), Some(month), Some(day)) => {
                if year < 0 {
                    Err(anyhow!("year must be positive with provided day and month"))
                } else {
                    ValidatedDate::from_ymd(year as u32, month, day).map_err(anyhow::Error::from)
                }
            }
            (None, None, None) => Ok(now),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod testing {
    use test_case::test_case;
    use chrono::NaiveDate;

    use super::*;

    #[test_case(Some(2), None, None, 1999, 7, 14 => (1999, 7, 16)  ; "Should advance date by 2 days")]
    #[test_case(Some(-3), None, None, 1662, 6, 2 => (1662, 5, 30)  ; "Should go back 3 days")]
    #[test_case(Some(2222), Some(42), None, 1, 1, 1 => (2222, 2, 11)  ; "Should match given year and exact day of this year.")]
    #[test_case(Some(2003), Some(8), Some(12), 1, 1, 1 => (2003, 8, 12)  ; "Should match extact date in year, month and day")]
    fn should_produce_valid_date(
        range_or_year: Option<i32>,
        day_of_year_or_month: Option<u32>,
        day_of_month: Option<u32>,
        given_y: i32,
        given_m: u32,
        given_d: u32,
    ) -> (u32, u32, u32) {
        let given = EditCommand {
            range_or_year,
            day_of_year_or_month,
            day_of_month,
        };
        let given_date: ValidatedDate = NaiveDate::from_ymd_opt(given_y, given_m, given_d)
            .expect("given_date has invalid year,month or day for a date")
            .into();

        let actual = given
            .to_advance_valid_date(given_date)
            .expect("Actual should result in error");

        (actual.year(), actual.month(), actual.day())
    }
}
