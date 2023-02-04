use std::fs;
use crate::core::dates_names::DateNameForFile;
use crate::prelude::*;
use crate::core::file_access;

use crate::core::app_options::AppOptions;
use super::date_models::open_by::OpenByMonthInYear;
use super::date_models::units_validated::{ValidatedDate, ValidatedYear};
use super::dates_names::yearly_name::YearlyName;
use super::dates_names::{DailyName, MonthlyName};

/// # Returns
/// If true is returned then a daily journal was deleted. If false then there was no daily
/// journal to be deleted
/// ## Errors
/// - could not even determine if a journal exits or could delte the found journal for some reason
/// for example permission
pub fn delete_day_journal(to_delete: ValidatedDate, option: &AppOptions) -> AppResult<bool> {
    let daily_name: DailyName = to_delete.into();
    delete_given_journal(daily_name, option)
}
/// # Returns
/// If true is returned then a monthly journal was deleted. If false then there was no montly
/// journal to be deleted
/// ## Errors
/// - could not even determine if a journal exits or could delte the found journal for some reason
/// for example permission
pub fn delete_month_journal(to_delete: OpenByMonthInYear, option: &AppOptions) -> AppResult<bool> {
    let daily_name: MonthlyName = MonthlyName::from_month_in_year(&to_delete)?;
    delete_given_journal(daily_name, option)
}
/// # Returns
/// If true is returned then a yearly journal was deleted. If false then there was no yearly
/// journal to be deleted
/// ## Errors
/// - could not even determine if a journal exits or could delte the found journal for some reason
/// for example permission
pub fn delete_year_journal(to_delete: ValidatedYear, option: &AppOptions) -> AppResult<bool> {
    let daily_name: YearlyName = YearlyName::new(to_delete);
    delete_given_journal(daily_name, option)
}

fn delete_given_journal<T>(journal: T, option: &AppOptions) -> AppResult<bool>
where
    T: DateNameForFile,
{
    let to_open = file_access::create_new_path_for(journal.name(), option)?;

    match to_open.try_exists() {
        Ok(does_exits) => {
            if does_exits {
                if to_open.is_file() {
                    fs::remove_file(to_open)?;
                    Ok(true)
                } else {
                    bail!("Critical: found directory where file as journal was expected to be deleted")
                }
            } else {
                // no journal found to delete
                Ok(false)
            }
        }
        Err(error) => Err(AppError::new(error).context(
            "Could not delete chosen journal. Could not event determine if a journal exits.",
        )),
    }
}
