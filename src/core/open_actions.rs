use std::fs;

use chrono::{Local, Datelike};
use crate::prelude::*;
use crate::core::{app_options::AppOptions, date_models::open_by::OpenByMonthInYear};
use super::{
    date_models::units_validated::{ValidatedDate, ValidatedYear},
    file_access, process_handling, DailyName,
    dates_names::{MonthlyName, DateNameForFile, yearly_name::YearlyName, InitialabeFromTemplate},
};

pub fn open_by_date(to_open_by: ValidatedDate, option: &AppOptions) -> AppResult {
    let today_name: DailyName = to_open_by.into();
    open_date_with_editor(today_name, option)
}

pub fn open_by_month_year(month_year: OpenByMonthInYear, option: &AppOptions) -> AppResult {
    let monthly = MonthlyName::from_month_in_year(&month_year)?;

    open_date_with_editor(monthly, option)
}
pub fn open_by_year(year: ValidatedYear, option: &AppOptions) -> AppResult {
    let yearly = YearlyName::new(year);

    open_date_with_editor(yearly, option)
}
pub fn open_by_current_year(option: &AppOptions) -> AppResult {
    let now = Local::now().date_naive().year() as u32;
    let year = now.try_into()?;
    let yearly = YearlyName::new(year);

    open_date_with_editor(yearly, option)
}

fn open_date_with_editor<T>(journal: T, option: &AppOptions) -> AppResult
where
    T: DateNameForFile + InitialabeFromTemplate,
{
    let to_open = file_access::create_new_path_for(journal.name(), option)?;

    if !to_open.exists() {
        let template_content = journal.try_get_template(option)?;
        if let Some(content) = template_content {
            fs::write(&to_open, content)?;
        }
    }
    process_handling::start_process_with(&to_open)?;

    Ok(())
}
