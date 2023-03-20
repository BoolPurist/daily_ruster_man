use std::{fs, io};
use std::path::Path;

use chrono::{Local, Datelike};
use crate::cli::edit_argument::EditCommonArgs;
use crate::core::template;
use crate::prelude::*;
use crate::core::{app_options::AppOptions, date_models::open_by::OpenByMonthInYear};
use super::app_config::AppConfig;
use super::process_handling::ProcessExecuter;
use super::{
    date_models::units_validated::{ValidatedDate, ValidatedYear},
    file_access, DailyName,
    dates_names::{MonthlyName, DateNameForFile, yearly_name::YearlyName, InitialabeFromTemplate},
};

pub type OpenResult = AppResult<Option<String>>;

pub fn open_by_date(
    process_executer: &impl ProcessExecuter,
    to_open_by: ValidatedDate,
    option: &AppOptions,
    edit_option: &EditCommonArgs,
) -> OpenResult {
    let today_name: DailyName = to_open_by.into();
    open_date_with_editor(process_executer, today_name, option, edit_option)
}

pub fn open_by_month_year(
    process_executer: &impl ProcessExecuter,
    month_year: OpenByMonthInYear,
    option: &AppOptions,
    edit_option: &EditCommonArgs,
) -> OpenResult {
    let monthly = MonthlyName::from_month_in_year(&month_year)?;

    open_date_with_editor(process_executer, monthly, option, edit_option)
}
pub fn open_by_year(
    process_executer: &impl ProcessExecuter,
    year: ValidatedYear,
    option: &AppOptions,
    edit_option: &EditCommonArgs,
) -> OpenResult {
    let yearly = YearlyName::new(year);

    open_date_with_editor(process_executer, yearly, option, edit_option)
}
pub fn open_by_current_year(
    process_executer: &impl ProcessExecuter,
    option: &AppOptions,
    edit_option: &EditCommonArgs,
) -> OpenResult {
    let now = Local::now().date_naive().year() as u32;
    let year = now.try_into()?;
    let yearly = YearlyName::new(year);

    open_date_with_editor(process_executer, yearly, option, edit_option)
}

fn open_date_with_editor<T>(
    process_executer: &impl ProcessExecuter,
    journal: T,
    option: &AppOptions,
    edit_option: &EditCommonArgs,
) -> AppResult<Option<String>>
where
    T: DateNameForFile + InitialabeFromTemplate,
{
    let to_open = file_access::create_new_path_for(journal.name(), option)?;

    let editor_to_use = edit_option.resolve_editor(option).unwrap_or_else(|error| {
        warn!(
            "Falling back to default editor {} due to error in loading config file correctly.\n {}",
            EditCommonArgs::DEFAUTL_EDITOR,
            error,
        );

        EditCommonArgs::DEFAUTL_EDITOR.to_owned()
    });

    if !to_open.exists() {
        info!("No journal created so far at {:?}", &to_open);
        try_write_template_from_config(&to_open, journal, option)?;
    }

    if edit_option.show_only() {
        return just_load_journal(&to_open);
    }

    process_executer.start_program(option, &editor_to_use, &to_open)?;

    return Ok(None);

    fn just_load_journal(to_open: &Path) -> OpenResult {
        match fs::read_to_string(to_open) {
            Ok(content) => Ok(Some(content)),
            Err(error) => {
                if error.kind() == io::ErrorKind::NotFound {
                    info!("No journal created to show so far");
                    Ok(None)
                } else {
                    Err(AppError::new(error))
                }
            }
        }
    }
}

fn try_write_template_from_config(
    to_open: &Path,
    journal: impl InitialabeFromTemplate,
    option: &AppOptions,
) -> AppResult {
    let config = option.load_config()?;
    if let Some(loaded) = config {
        if let Some(path) = journal.choose_template(loaded).try_to_resolved_path(loaded) {
            let template_content = try_create_template(loaded, &path)?;
            if let Some(content) = template_content {
                debug!("Used template content:\n{}", content);
                if !option.run_editor_dry() {
                    fs::write(to_open, content)?;
                }
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
