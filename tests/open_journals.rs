mod common;
use std::path::PathBuf;

use common::FileTmpBuilder;
use daily_ruster_man::{
    core::{
        open_actions, process_handling::TestProcessExecuter,
        date_models::units_validated::ValidatedDate, app_options::AppOptions,
    },
    cli::{app_args::GenerellArgs, edit_argument::EditCommonArgs},
};
use tempfile::TempDir;

#[test]
fn should_open_specific_daily_journal() {
    let processor = TestProcessExecuter::default();
    let files = FileTmpBuilder::default().build();

    let date = ValidatedDate::new(
        2022.try_into().unwrap(),
        11.try_into().unwrap(),
        10.try_into().unwrap(),
    )
    .expect("Invalid date");

    let app_options = AppOptions::with(GenerellArgs::new(
        false,
        None,
        Some(files.path().to_str().unwrap().to_string()),
    ));
    let edit_option = EditCommonArgs::default();
    let actual = open_actions::open_by_date(&processor, date, &app_options, &edit_option);

    assert_open_action(
        processor,
        files,
        matches!(actual, Ok(None)),
        "2022_11_10_daily.md".into(),
        "vim",
    );
}

#[test]
fn should_open_specific_daily_journal_with_specific_editor() {
    const EXPECTED_EDITOR: &str = "expected_editor";

    let processor = TestProcessExecuter::default();
    let files = FileTmpBuilder::default().build();

    let date = ValidatedDate::new(
        2022.try_into().unwrap(),
        11.try_into().unwrap(),
        10.try_into().unwrap(),
    )
    .expect("Invalid date");

    let app_options = AppOptions::with(GenerellArgs::new(
        false,
        None,
        Some(files.path().to_str().unwrap().to_string()),
    ));
    let edit_option = EditCommonArgs::new(Some(EXPECTED_EDITOR.to_owned()), false);

    let actual = open_actions::open_by_date(&processor, date, &app_options, &edit_option);

    assert_open_action(
        processor,
        files,
        matches!(actual, Ok(None)),
        "2022_11_10_daily.md".into(),
        EXPECTED_EDITOR,
    );
}

#[test]
fn should_open_specific_daily_journal_with_show_only() {
    let expected_content = Some("Some content".to_owned());

    let processor = TestProcessExecuter::default();
    let files = FileTmpBuilder::default()
        .with_file(
            PathBuf::from("2024_09_07_daily.md"),
            expected_content.clone(),
        )
        .build();

    let date = ValidatedDate::new(
        2024.try_into().unwrap(),
        9.try_into().unwrap(),
        7.try_into().unwrap(),
    )
    .expect("Invalid date");

    let app_options = AppOptions::with(GenerellArgs::new(
        false,
        None,
        Some(files.path().to_str().unwrap().to_string()),
    ));
    let edit_option = EditCommonArgs::new(None, true);

    let actual = open_actions::open_by_date(&processor, date, &app_options, &edit_option);

    assert!(matches!(actual, Ok(actual_content) if actual_content == expected_content));
    assert!(processor.get_last_executed_program().is_empty());
}

#[test]
fn should_not_return_nothing_for_empty_with_show_only() {
    let processor = TestProcessExecuter::default();
    let files = FileTmpBuilder::default().build();

    let date = ValidatedDate::new(
        2024.try_into().unwrap(),
        9.try_into().unwrap(),
        7.try_into().unwrap(),
    )
    .expect("Invalid date");

    let app_options = AppOptions::with(GenerellArgs::new(
        false,
        None,
        Some(files.path().to_str().unwrap().to_string()),
    ));
    let edit_option = EditCommonArgs::new(None, true);

    let actual = open_actions::open_by_date(&processor, date, &app_options, &edit_option);

    assert!(matches!(actual, Ok(None)));
    assert!(processor.get_last_executed_program().is_empty());
}
fn assert_open_action(
    processor: TestProcessExecuter,
    files: TempDir,
    match_on_return: bool,
    expected_file_name: PathBuf,
    expected_editor: &str,
) {
    let expected_path = files.path().join(&expected_file_name);
    let expected_execution = format!("{} {}", expected_editor, expected_path.to_string_lossy());
    let actual_execution = processor.get_last_executed_program();

    assert!(match_on_return);
    assert_eq!(expected_execution, actual_execution);
}
