mod daily_filtering;
mod daily_names;
use crate::AppResult;
pub mod data_models;
use data_models::*;
mod file_access;
mod process_handling;
use self::daily_names::DailyName;
pub fn open_today() -> AppResult {
    let today_name = DailyName::create_today_name();
    let to_open = file_access::create_new_path_for(today_name.name());

    process_handling::start_process_with(&to_open);
    Ok(())
}

pub fn fetch_all_daily_names() -> Vec<String> {
    let filter_by_valid_format = fetch_valid_daily_entries();
    filter_by_valid_format
        .into_iter()
        .map(|str| str.to_string())
        .collect()
}

fn fetch_valid_daily_entries() -> Vec<DailyName> {
    let daily_paths = file_access::get_all_daily_paths();

    let file_names = daily_filtering::strip_expect_file_name(&daily_paths);

    daily_filtering::filter_out_non_daily(file_names).collect()
}

pub fn open_by_future_past_range(range: &PastFuture) -> AppResult {
    let wanted_daily = DailyName::create_from_range(range);

    let to_open = file_access::create_new_path_for(wanted_daily.name());
    process_handling::start_process_with(&to_open);

    Ok(())
}
pub fn open_by_day_of_year(day_of_year: &DayOfYear) -> AppResult {
    let wanted_daily = DailyName::create_from_ordinal_day(day_of_year)?;

    let to_open = file_access::create_new_path_for(wanted_daily.name());
    process_handling::start_process_with(&to_open);

    Ok(())
}
pub fn open_by_year_month_day(ymd: &DayMonthYear) -> AppResult {
    let wanted_daily = DailyName::creaet_from_year_month_day(ymd)?;

    let to_open = file_access::create_new_path_for(wanted_daily.name());
    process_handling::start_process_with(&to_open);

    Ok(())
}
