use crate::prelude::*;
use crate::core::date_models::open_by::OpenByMonthInYear;
use super::{
    date_models::units_validated::ValidatedDate,
    file_access, process_handling, DailyName,
    dates_names::{MonthlyName, DateNameForFile},
};

pub fn open_by_date(to_open_by: ValidatedDate) -> AppResult {
    let today_name: DailyName = to_open_by.into();

    open_date_with_editor(today_name.name())
}
pub fn open_by_month_year(month_year: OpenByMonthInYear) -> AppResult {
    let monthly = MonthlyName::from_month_in_year(&month_year)?;

    open_date_with_editor(monthly.name())
}

fn open_date_with_editor(name_journal: &str) -> AppResult {
    let to_open = file_access::create_new_path_for(name_journal)?;
    process_handling::start_process_with(&to_open)?;

    Ok(())
}
