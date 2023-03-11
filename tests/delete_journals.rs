mod common;
use std::path::PathBuf;

use daily_ruster_man::AppResult;
use daily_ruster_man::core::app_options::AppOptions;
use daily_ruster_man::core::date_models::open_by::OpenByMonthInYear;
use daily_ruster_man::core::delete_actions::{self, DeletionResult};
use daily_ruster_man::cli::{app_args::GenerellArgs, deletion_arguments::CommonDeleteArg};

use daily_ruster_man::core::date_models::units_validated::{
    ValidatedYear, ValidatedMonth, ValidatedDate, ValidatedDay,
};
use tempfile::TempDir;

#[test]
fn should_delete_yearly_journal() {
    const TO_DELETE: &str = "2022_yearly.md";
    let provided_set_up = set_up(TO_DELETE, true);

    let year: ValidatedYear = 2022.try_into().unwrap();

    // Act
    let result = delete_actions::delete_year_journal(
        year,
        &provided_set_up.common,
        &AppOptions::with(provided_set_up.general.clone()),
    );

    assert_deletion(result, provided_set_up);
}

#[test]
fn should_delete_monthly_journal() {
    const TO_DELETE: &str = "2001_01_monthly.md";

    let provided_set_up = set_up(TO_DELETE, true);

    let month: ValidatedMonth = 1.try_into().unwrap();
    let year: ValidatedYear = 2001.try_into().unwrap();

    // Act
    let result = delete_actions::delete_month_journal(
        OpenByMonthInYear::WithYear { month, year },
        &provided_set_up.common,
        &AppOptions::with(provided_set_up.general.clone()),
    );

    assert_deletion(result, provided_set_up);
}

#[test]
fn should_delete_daily_journal() {
    const TO_DELETE: &str = "1988_11_22_daily.md";

    let provided_set_up = set_up(TO_DELETE, true);

    let month: ValidatedMonth = 11.try_into().unwrap();
    let year: ValidatedYear = 1988.try_into().unwrap();
    let day: ValidatedDay = 22.try_into().unwrap();
    let date: ValidatedDate = ValidatedDate::new(year, month, day).unwrap();
    // Act
    let result = delete_actions::delete_day_journal(
        date,
        &provided_set_up.common,
        &AppOptions::with(provided_set_up.general.clone()),
    );

    assert_deletion(result, provided_set_up);
}
#[test]
fn should_return_no_daily_found_for_deletion() {
    const TO_DELETE: &str = "1988_1_22_daily.md";

    let provided_set_up = set_up(TO_DELETE, false);

    let month: ValidatedMonth = 1.try_into().unwrap();
    let year: ValidatedYear = 1988.try_into().unwrap();
    let day: ValidatedDay = 22.try_into().unwrap();
    let date: ValidatedDate = ValidatedDate::new(year, month, day).unwrap();
    // Act
    let result = delete_actions::delete_day_journal(
        date,
        &provided_set_up.common,
        &AppOptions::with(provided_set_up.general.clone()),
    );

    assert_no_deletion_happened(result, provided_set_up);
}
#[test]
fn should_return_no_monthly_found_for_deletion() {
    const TO_DELETE: &str = "2001_06_monthly.md";

    let provided_set_up = set_up(TO_DELETE, false);

    let month: ValidatedMonth = 6.try_into().unwrap();
    let year: ValidatedYear = 2001.try_into().unwrap();

    // Act
    let result = delete_actions::delete_month_journal(
        OpenByMonthInYear::WithYear { month, year },
        &provided_set_up.common,
        &AppOptions::with(provided_set_up.general.clone()),
    );

    assert_no_deletion_happened(result, provided_set_up);
}

fn set_up(to_delete: &str, should_exit_before: bool) -> DeletionSetup {
    let files = common::create_sample_data_folder();
    let path_to_delete: PathBuf = files.path().join(to_delete);

    let does_exists = path_to_delete.exists();
    if should_exit_before {
        assert!(
            does_exists,
            "Precondition: no path to delete exits in the 1. place."
        );
    } else {
        assert!(
            !does_exists,
            "Precondition: Paht should not exit in the first place."
        );
    }

    let common = CommonDeleteArg::new(true);
    let general = GenerellArgs::new(
        false,
        None,
        Some(files.path().to_str().unwrap().to_string()),
    );

    DeletionSetup {
        _files: files,
        path_to_delete,
        general,
        common,
    }
}

fn assert_deletion(result: AppResult<DeletionResult>, provided_set_up: DeletionSetup) {
    // Assert
    if let Ok(deletion_outcome) = result {
        assert!(DeletionResult::Deleted == deletion_outcome);
    } else {
        panic!("Should have returned that the journal was deleted.");
    }

    assert!(!provided_set_up.path_to_delete.exists());
}
fn assert_no_deletion_happened(result: AppResult<DeletionResult>, provided_set_up: DeletionSetup) {
    // Assert
    if let Ok(deletion_outcome) = result {
        assert!(DeletionResult::NoJournalFound == deletion_outcome);
    } else {
        panic!("Should have returned that there was no journal to deleted.");
    }

    assert!(!provided_set_up.path_to_delete.exists());
}

struct DeletionSetup {
    _files: TempDir,
    pub common: CommonDeleteArg,
    pub general: GenerellArgs,
    pub path_to_delete: PathBuf,
}
