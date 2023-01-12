use crate::prelude::*;
use super::{check_for_month, check_for_day};

#[derive(CopyGetters, Debug)]
#[getset(get_copy = "pub")]
pub struct FindByYearMonthDay {
    year: Option<u32>,
    month: Option<u32>,
    day: Option<u32>,
}

pub enum FindByMonthInYear {
    All,
    InCurrentYear(u32),
    MonthYear { month: u32, year: u32 },
}

impl FindByYearMonthDay {
    pub fn new(y_opt: Option<u32>, m_opt: Option<u32>, d_opt: Option<u32>) -> AppResult<Self> {
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

        Ok(Self {
            year: y_opt,
            month: m_opt,
            day: d_opt,
        })
    }
}
