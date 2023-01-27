use chrono::{Local, Datelike};
use crate::core::conf::AppConfig;
use std::fs;
use crate::prelude::*;

use crate::core::date_models::open_by::OpenByMonthInYear;
use super::{
    date_models::units_validated::{ValidatedDate, ValidatedYear},
    file_access::{self, LoadedAppConfig},
    process_handling, DailyName,
    dates_names::{MonthlyName, DateNameForFile, yearly_name::YearlyName},
};

pub fn open_by_date(to_open_by: ValidatedDate) -> AppResult {
    let today_name: DailyName = to_open_by.into();
    let template_content =
        try_load_and_get_template(|app_conf| app_conf.daily_template().as_deref())?;
    open_date_with_editor(today_name.name(), template_content)
}

pub fn open_by_month_year(month_year: OpenByMonthInYear) -> AppResult {
    let monthly = MonthlyName::from_month_in_year(&month_year)?;

    let template_content =
        try_load_and_get_template(|app_conf| app_conf.monthly_template().as_deref())?;

    open_date_with_editor(monthly.name(), template_content)
}
pub fn open_by_year(year: ValidatedYear) -> AppResult {
    let yearly = YearlyName::new(year);

    let template_content =
        try_load_and_get_template(|app_conf| app_conf.yearly_template().as_deref())?;

    open_date_with_editor(yearly.name(), template_content)
}
pub fn open_by_current_year() -> AppResult {
    let now = Local::now().date_naive().year() as u32;
    let year = now.try_into()?;
    let yearly = YearlyName::new(year);

    let template_content =
        try_load_and_get_template(|app_conf| app_conf.yearly_template().as_deref())?;

    open_date_with_editor(yearly.name(), template_content)
}

fn try_load_and_get_template(
    on_get_template: impl Fn(&AppConfig) -> Option<&str>,
) -> AppResult<Option<String>> {
    let loaded_config = file_access::fetch_path_conf_file_content()?;
    try_get_template_file(&loaded_config, on_get_template)
}

fn try_get_template_file(
    loaded_config: &LoadedAppConfig,
    on_get_template: impl Fn(&AppConfig) -> Option<&str>,
) -> AppResult<Option<String>> {
    if let Some(conf) = loaded_config.config_content() {
        if let Some(template_path) = on_get_template(conf) {
            let path = loaded_config.root().join(template_path);
            if path.exists() {
                let template_content = fs::read_to_string(&path)?;
                info!("Template path found at {:?}", path);
                Ok(Some(template_content))
            } else {
                info!("No template found at {:?}", path);
                Ok(None)
            }
        } else {
            info!("No template path found in config");
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

fn open_date_with_editor(name_journal: &str, initial_content: Option<String>) -> AppResult {
    let to_open = file_access::create_new_path_for(name_journal)?;

    if let Some(content) = initial_content {
        if !to_open.exists() {
            if let Err(error) = std::fs::write(&to_open, content) {
                warn!(
                    "Could write template to new entry {:?}. Error: {:?}",
                    to_open, error
                );
            }
        }
    }

    process_handling::start_process_with(&to_open)?;

    Ok(())
}
