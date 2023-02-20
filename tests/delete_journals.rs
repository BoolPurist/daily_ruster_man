mod common;
use std::path::PathBuf;

use daily_ruster_man::core::app_options::AppOptions;
use daily_ruster_man::core::date_models::open_by::OpenByMonthInYear;
use daily_ruster_man::core::delete_actions::{self, DeletionResult};
use daily_ruster_man::cli::{app_args::GenerellArgs, deletion_arguments::CommonDeleteArg};

use daily_ruster_man::core::date_models::units_validated::{ValidatedYear, ValidatedMonth};
use tempfile::TempDir;

#[test]
fn should_delete_yearly_journal() {
    const TO_DELETE: &str = "2022_yearly.md";
    let provided_set_up = set_up(TO_DELETE);

    let year: ValidatedYear = 2022.try_into().unwrap();

    // Act
    let result = delete_actions::delete_year_journal(
        year,
        &provided_set_up.common,
        &AppOptions::with(provided_set_up.general),
    );

    assert!(result.is_ok());
    assert!(!provided_set_up.path_to_delete.exists());
}

#[test]
fn should_delete_monthly_journal() {
    const TO_DELETE: &str = "2001_01_monthly.md";

    let provided_set_up = set_up(TO_DELETE);

    let month: ValidatedMonth = 1.try_into().unwrap();
    let year: ValidatedYear = 2001.try_into().unwrap();

    // Act
    let result = delete_actions::delete_month_journal(
        OpenByMonthInYear::WithYear { month, year },
        &provided_set_up.common,
        &AppOptions::with(provided_set_up.general),
    );

    // Assert
    if let Ok(deletion_outcome) = result {
        assert!(DeletionResult::Deleted == deletion_outcome);
    } else {
        panic!("Should have returned that the journal was deleted.");
    }

    assert!(!provided_set_up.path_to_delete.exists());
}

fn set_up(to_delete: &str) -> DeletionSetup {
    let files = common::create_sample_data_folder();
    let path_to_delete: PathBuf = files.path().join(to_delete);

    assert!(
        path_to_delete.exists(),
        "Precondition: no path to delete exits in the 1. place."
    );

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

struct DeletionSetup {
    _files: TempDir,
    pub common: CommonDeleteArg,
    pub general: GenerellArgs,
    pub path_to_delete: PathBuf,
}
