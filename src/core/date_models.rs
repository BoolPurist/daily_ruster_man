use crate::prelude::*;
#[derive(Debug)]
pub enum ByDaysInTime {
    Past(u32),
    Future(u32),
}

impl ByDaysInTime {
    pub fn new(past_or_future: i32) -> Self {
        if past_or_future < 0 {
            Self::Past(past_or_future.unsigned_abs())
        } else {
            Self::Future(past_or_future.unsigned_abs())
        }
    }
}

#[derive(new, CopyGetters, Debug)]
#[getset(get_copy = "pub")]
pub struct DayOfYear {
    year: u32,
    day_of_year: u32,
}
#[derive(new, CopyGetters, Debug)]
#[getset(get_copy = "pub")]
pub struct DayMonthYear {
    year: u32,
    month: u32,
    day: u32,
}

#[derive(Debug)]
pub enum MonthInYear {
    CurrentMonth,
    InCurrentYear(u32),
    WithYear { month: u32, year: u32 },
}

#[derive(CopyGetters, Debug)]
#[getset(get_copy = "pub")]
pub struct FilterParamsYmD {
    year: Option<u32>,
    month: Option<u32>,
    day: Option<u32>,
}

pub type Optymd = Option<u32>;
impl FilterParamsYmD {
    pub fn new(y_opt: Optymd, m_opt: Optymd, d_opt: Optymd) -> AppResult<Self> {
        match (y_opt, m_opt, d_opt) {
            (Some(y), Some(m), Some(d)) => {
                if chrono::NaiveDate::from_ymd_opt(y as i32, m, d).is_none() {
                    bail!(
                        "year, month or day is not valid for given date: year: {y}, month: {m}, day: {d}"
                    )
                }
            }
            (_, Some(m), Some(d)) => {
                check_for_day(d)?;
                check_for_month(m)?;
            }
            (_, Some(m), None) => {
                check_for_month(m)?;
            }
            (_, None, Some(d)) => {
                check_for_day(d)?;
            }
            _ => (),
        };

        return Ok(Self {
            year: y_opt,
            month: m_opt,
            day: d_opt,
        });
    }
}

pub fn check_for_month(m: u32) -> AppResult {
    const LOWER_BOUND: u32 = 1;
    const UPPER_BOUND: u32 = 12;

    if !(LOWER_BOUND..=UPPER_BOUND).contains(&m) {
        bail!("Month must be between {LOWER_BOUND} and {UPPER_BOUND}.")
    }

    Ok(())
}

pub fn check_for_day(d: u32) -> AppResult {
    const LOWER_BOUND: u32 = 1;
    const UPPER_BOUND: u32 = 31;

    if !(LOWER_BOUND..=UPPER_BOUND).contains(&d) {
        bail!("Day of month must be between {LOWER_BOUND} and {UPPER_BOUND}.")
    }

    Ok(())
}
