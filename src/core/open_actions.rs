use chrono::{Local, Datelike};

use crate::prelude::*;
use crate::core::date_models::open_by::OpenByMonthInYear;
use super::{
    date_models::units_validated::{ValidatedDate, ValidatedYear},
    file_access, process_handling, DailyName,
    dates_names::{MonthlyName, DateNameForFile, yearly_name::YearlyName},
};

pub fn open_by_date(to_open_by: ValidatedDate) -> AppResult {
    let today_name: DailyName = to_open_by.into();

    open_date_with_editor(today_name.name(), "day")
}
pub fn open_by_month_year(month_year: OpenByMonthInYear) -> AppResult {
    let monthly = MonthlyName::from_month_in_year(&month_year)?;

    open_date_with_editor(monthly.name(), "day")
}
pub fn open_by_year(year: ValidatedYear) -> AppResult {
    let yearly = YearlyName::new(year);

    open_date_with_editor(yearly.name(), "year")
}
pub fn open_by_current_year() -> AppResult {
    let now = Local::now().date_naive().year() as u32;
    let year = now.try_into()?;
    let yearly = YearlyName::new(year);

    open_date_with_editor(yearly.name(), "year")
}

fn open_date_with_editor(name_journal: &str, initial_content: &str) -> AppResult {
    let to_open = file_access::create_new_path_for(name_journal)?;

    if !to_open.exists() {
        if let Err(error) = std::fs::write(&to_open, initial_content) {
            warn!(
                "Could write template to new entry {:?}. Error: {:?}",
                to_open, error
            );
        }
    }

    process_handling::start_process_with(&to_open)?;

    Ok(())
}
