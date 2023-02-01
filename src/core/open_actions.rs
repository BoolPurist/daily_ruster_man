use std::fs;
use std::path::Path;

use chrono::{Local, Datelike};
use crate::core::template;
use crate::prelude::*;
use crate::core::{app_options::AppOptions, date_models::open_by::OpenByMonthInYear};
use super::app_config::AppConfig;
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
        try_write_template_from_config(&to_open, journal, option)?;
    }
    process_handling::start_process_with(&to_open)?;

    Ok(())
}

fn try_write_template_from_config(
    to_open: &Path,
    journal: impl InitialabeFromTemplate,
    option: &AppOptions,
) -> AppResult {
    let config = option.load_config()?;
    if let Some(loaded) = config {
        if let Some(path) = journal.choose_template(loaded) {
            let template_content = try_create_template(loaded, Path::new(path))?;
            if let Some(content) = template_content {
                debug!("Used template content:\n{}", content);
                fs::write(to_open, content)?;
                return Ok(());
            }
        }
    }

    Ok(())
}

/// Returns none if there is no template file at the given parameter.
fn try_create_template(app_config: &AppConfig, template_path: &Path) -> AppResult<Option<String>> {
    debug!("Augmenting template with placeholders from config file");
    let mut placeholders = app_config.create_placeholder_for_template();
    let maybe_template_content = app_config.try_get_template_file_content(template_path)?;
    if let Some(content) = maybe_template_content {
        let augmented_with_placeholders =
            template::replace_template_placeholders(&content, &mut placeholders);

        for (key, error_msg) in augmented_with_placeholders.errors().iter() {
            error!(
                "For key {} the command was executed with errors.\nError: {}",
                key, error_msg
            );
        }
        Ok(Some(augmented_with_placeholders.replacement().to_owned()))
    } else {
        Ok(None)
    }
}
