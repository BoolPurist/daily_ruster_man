pub mod find_by;
pub mod open_by;
pub mod units;
use crate::{
    prelude::*,
    core::constants::{MONTH_LOWER_BOUND, MONTH_UPPER_BOUND, DAY_LOWER_BOUND, DAY_UPPER_BOUND},
};

pub fn check_for_month(m: u32) -> AppResult {
    if !(MONTH_LOWER_BOUND..=MONTH_UPPER_BOUND).contains(&m) {
        bail!(
            "Month must be between {} and {}.",
            MONTH_LOWER_BOUND,
            MONTH_UPPER_BOUND
        )
    }

    Ok(())
}

pub fn check_for_day(d: u32) -> AppResult {
    if !(DAY_LOWER_BOUND..=DAY_UPPER_BOUND).contains(&d) {
        bail!(
            "Day of month must be between {} and {}.",
            DAY_LOWER_BOUND,
            DAY_UPPER_BOUND
        )
    }

    Ok(())
}
