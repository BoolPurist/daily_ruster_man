use chrono::{NaiveDate, Days};

use crate::AppResult;
use date_validation_types::{ValidatedDate, ValidatedMonth, ValidatedYear};

#[derive(Debug)]
pub enum OpenByDaysInTime {
    Past(u32),
    Future(u32),
}

impl OpenByDaysInTime {
    pub fn new(past_or_future: i32) -> Self {
        if past_or_future < 0 {
            Self::Past(past_or_future.unsigned_abs())
        } else {
            Self::Future(past_or_future.unsigned_abs())
        }
    }
}

impl OpenByDaysInTime {
    pub fn from_point_in_time(&self, date: ValidatedDate) -> AppResult<ValidatedDate> {
        let date = match self {
            Self::Past(past) => {
                let date: NaiveDate = date.into();
                date.checked_sub_days(Days::new(*past as u64))
                    .ok_or_else(|| {
                        anyhow!(
                            "No valid date if date {1} would be {0} days in the past",
                            past,
                            date
                        )
                    })
            }
            Self::Future(future) => {
                let date: NaiveDate = date.into();
                date.checked_add_days(Days::new(*future as u64))
                    .ok_or_else(|| {
                        anyhow!(
                            "No valid date if date {1} would be {0} days in the future",
                            future,
                            date
                        )
                    })
            }
        }?;

        Ok(date.into())
    }
}

#[derive(Debug)]
pub enum OpenByMonthInYear {
    CurrentMonth,
    InCurrentYear(ValidatedMonth),
    WithYear {
        month: ValidatedMonth,
        year: ValidatedYear,
    },
}
