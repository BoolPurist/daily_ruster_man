use super::{
    data_models::{DayMonthYear, DayOfYear, PastFuture},
    file_access, process_handling, DailyName,
};

use crate::AppResult;
pub fn open_today() -> AppResult {
    let today_name = DailyName::create_today_name();
    let to_open = file_access::create_new_path_for(today_name.name());

    process_handling::start_process_with(&to_open);
    Ok(())
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
