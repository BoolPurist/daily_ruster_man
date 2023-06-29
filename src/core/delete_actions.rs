use std::fs;
use crate::cli::prompt;
use crate::core::dates_names::DateNameForFile;
use crate::prelude::*;
use crate::core::file_access;

use crate::core::app_options::AppOptions;
use crate::cli::deletion_arguments::*;
use super::date_models::open_by::OpenByMonthInYear;
use date_validation_types::{ValidatedDate, ValidatedYear};
use super::dates_names::yearly_name::YearlyName;
use super::dates_names::{DailyName, MonthlyName};

#[derive(PartialEq, Eq)]
pub enum DeletionResult {
    /// There was journal to be found for deletion
    NoJournalFound,
    /// Use did cancel the deletion when asked for confirmation
    NoConfirmation,
    /// User confirmed and the journal was deleted
    Deleted,
}

/// ## Errors
/// - could not even determine if a journal exits or could delte the found journal for some reason
/// for example permission
pub fn delete_day_journal(
    to_delete: ValidatedDate,
    deletion_option: &CommonDeleteArg,
    option: &AppOptions,
) -> AppResult<DeletionResult> {
    let daily_name: DailyName = to_delete.into();
    delete_given_journal(daily_name, deletion_option, option, || {
        prompt::ask_for_confirmation("you want to delete the chosen daily journal ?")
    })
}
/// ## Errors
/// - could not even determine if a journal exits or could delte the found journal for some reason
/// for example permission
pub fn delete_month_journal(
    to_delete: OpenByMonthInYear,
    deletion_option: &CommonDeleteArg,
    option: &AppOptions,
) -> AppResult<DeletionResult> {
    let daily_name: MonthlyName = MonthlyName::from_month_in_year(&to_delete)?;
    delete_given_journal(daily_name, deletion_option, option, || {
        prompt::ask_for_confirmation("you want to delete the chosen monthly journal ?")
    })
}

/// ## Errors
/// - could not even determine if a journal exits or could delte the found journal for some reason
/// for example permission
pub fn delete_year_journal(
    to_delete: ValidatedYear,
    deletion_option: &CommonDeleteArg,
    option: &AppOptions,
) -> AppResult<DeletionResult> {
    let daily_name: YearlyName = YearlyName::new(to_delete);
    delete_given_journal(daily_name, deletion_option, option, || {
        prompt::ask_for_confirmation("you want to delete the chosen yearly journal ?")
    })
}

fn delete_given_journal<T>(
    journal: T,
    deletion_option: &CommonDeleteArg,
    option: &AppOptions,
    on_confirmation: impl Fn() -> AppResult<bool>,
) -> AppResult<DeletionResult>
where
    T: DateNameForFile,
{
    let to_open = file_access::create_new_path_for(journal.name(), option)?;

    match to_open.try_exists() {
        Ok(does_exits) => {
            if does_exits {
                if to_open.is_file() {
                    let wants_to_delete = if !deletion_option.skip_confirmation() {
                        on_confirmation()
                    } else {
                        Ok(true)
                    }?;

                    if wants_to_delete {
                        fs::remove_file(to_open)?;
                        Ok(DeletionResult::Deleted)
                    } else {
                        Ok(DeletionResult::NoConfirmation)
                    }
                } else {
                    bail!("Critical: found directory where file as journal was expected to be deleted")
                }
            } else {
                // no journal found to delete
                Ok(DeletionResult::NoJournalFound)
            }
        }
        Err(error) => Err(AppError::new(error).context(
            "Could not delete chosen journal. Could not event determine if a journal exits.",
        )),
    }
}
