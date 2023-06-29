#[cfg(test)]
mod common;
use date_validation_types::{ValidatedYear, ValidatedMonth};
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
fn should_list_all_daily_journals_in_certain_year() {
    const YEAR: u32 = 2023;

    let querry = FindByYearMonthDay::new(Some(YEAR), None, None)
        .unwrap_or_else(|_| panic!("Year {} not a valid year", YEAR));

    let set_up = set_up_app_options();
    let all_daily_journals = list_queries::fetch_all_daily_names(&querry, &set_up.app_options)
        .unwrap_or_else(|_| panic!("Could not fetch all daily journals in year {}", YEAR));

    insta::assert_yaml_snapshot!(all_daily_journals);
}

#[test]
fn should_list_exact_daily_journal_by_date() {
    let querry = FindByYearMonthDay::new(Some(2023), Some(3), Some(8))
        .expect("Invalid date for querry provided");

    let set_up = set_up_app_options();
    let exact_daily_journal = list_queries::fetch_all_daily_names(&querry, &set_up.app_options)
        .expect("Could not exact daily journal");

    insta::assert_yaml_snapshot!(exact_daily_journal);
}

#[test]
fn should_list_all_daily_journal_in_months() {
    let querry =
        FindByYearMonthDay::new(Some(2023), Some(3), None).expect("Invalid year and month");

    let set_up = set_up_app_options();
    let daily_journals_in_month = list_queries::fetch_all_daily_names(&querry, &set_up.app_options)
        .expect("Could not fetch all daily journals in certain month");

    insta::assert_yaml_snapshot!(daily_journals_in_month);
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

#[test]
fn should_list_monthly_journals_in_certain_year() {
    let current_year: ValidatedYear = 2002.try_into().expect("Invalid year provided");
    let querry = FindByMonthInYear::InCurrentYear(current_year);

    let set_up = set_up_app_options();
    let all_monthly_journals = list_queries::fetch_all_monthly_names(&querry, &set_up.app_options)
        .expect("Could not fetch all monthly journals");

    insta::assert_yaml_snapshot!(all_monthly_journals);
}

#[test]
fn should_list_exact_monthly_journals() {
    let year: ValidatedYear = 2002.try_into().expect("Invalid year provided");
    let month: ValidatedMonth = 11.try_into().expect("Invalid month");

    let querry = FindByMonthInYear::MonthYear { month, year };

    let set_up = set_up_app_options();
    let all_monthly_journals = list_queries::fetch_all_monthly_names(&querry, &set_up.app_options)
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
