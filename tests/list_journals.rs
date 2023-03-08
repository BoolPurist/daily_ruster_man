mod common;
use daily_ruster_man::{
    core::{date_models::find_by::FindByYearMonthDay, app_options::AppOptions, list_queries},
    cli::app_args::GenerellArgs,
};

#[test]
fn should_list_all_daily_journals() {
    let files = common::create_sample_data_folder();
    let querry =
        FindByYearMonthDay::new(None, None, None).expect("Invalid date for querry provided");
    let general = GenerellArgs::new(
        false,
        None,
        Some(files.path().to_str().unwrap().to_string()),
    );

    let app_options = AppOptions::with(general);
    let all_daily_journals = list_queries::fetch_all_daily_names(&querry, &app_options)
        .expect("Could not fetch all daily journals");

    insta::assert_yaml_snapshot!(all_daily_journals);
}
