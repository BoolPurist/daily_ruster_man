mod common;
use std::path::PathBuf;

use daily_ruster_man::core::app_options::AppOptions;
use daily_ruster_man::core::delete_actions;
use daily_ruster_man::cli::{app_args::GenerellArgs, deletion_arguments::CommonDeleteArg};

use daily_ruster_man::core::date_models::units_validated::ValidatedYear;

#[test]
fn should_delete_yearly_journal() {
    const TO_DELETE: &str = "2022_yearly.md";
    let files = common::create_sample_data_folder();
    let path_to_delete: PathBuf = files.path().join(TO_DELETE);

    assert!(path_to_delete.exists());
    let common = CommonDeleteArg::new(true);
    let year: ValidatedYear = 2022.try_into().unwrap();

    let general = GenerellArgs::new(
        false,
        None,
        Some(files.path().to_str().unwrap().to_string()),
    );
    let result = delete_actions::delete_year_journal(year, &common, &AppOptions::with(general));

    assert!(result.is_ok());
    assert!(!path_to_delete.exists());
}
