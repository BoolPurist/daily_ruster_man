mod common;
use daily_ruster_man::{
    core::{
        date_models::find_by::{FindByYearMonthDay, FindByMonthInYear},
        app_options::AppOptions,
        list_queries,
    },
    cli::app_args::GenerellArgs,
};
use tempfile::TempDir;

#[test]
fn should_list_all_daily_journals() {
    let querry =
        FindByYearMonthDay::new(None, None, None).expect("Invalid date for querry provided");

    let set_up = set_up_app_options();
    let all_daily_journals = list_queries::fetch_all_daily_names(&querry, &set_up.app_options)
        .expect("Could not fetch all daily journals");

    insta::assert_yaml_snapshot!(all_daily_journals);
}

#[test]
fn should_list_all_monthly_journals() {
    let querry = FindByMonthInYear::All;

    let set_up = set_up_app_options();
    let all_monthly_journals = list_queries::fetch_all_monthly_names(&querry, &set_up.app_options)
        .expect("Could not fetch all monthly journals");

    insta::assert_yaml_snapshot!(all_monthly_journals);
}

#[test]
fn should_list_all_yearly_journals() {
    let set_up = set_up_app_options();
    let all_monthly_journals = list_queries::fetch_yearly_names(&set_up.app_options)
        .expect("Could not fetch all monthly journals");

    insta::assert_yaml_snapshot!(all_monthly_journals);
}

struct SetUpForListingQuerry {
    _files: TempDir,
    app_options: AppOptions,
}

fn set_up_app_options() -> SetUpForListingQuerry {
    let files = common::create_sample_data_folder();
    let app_options = AppOptions::with(GenerellArgs::new(
        false,
        None,
        Some(files.path().to_str().unwrap().to_string()),
    ));

    SetUpForListingQuerry {
        _files: files,
        app_options,
    }
}
