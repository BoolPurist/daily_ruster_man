use super::{
    data_models::{DayMonthYear, DayOfYear, PastFuture},
    file_access, process_handling, DailyName,
};

use crate::AppResult;
pub fn open_today() -> AppResult {
    let today_name = DailyName::create_today_name();

    open_daily_with_editor(&today_name)
}

pub fn open_by_future_past_range(range: &PastFuture) -> AppResult {
    debug!("open folder by looking at range {range:?}");
    let wanted_daily = DailyName::create_from_range(range);

    open_daily_with_editor(&wanted_daily)
}
pub fn open_by_day_of_year(day_of_year: &DayOfYear) -> AppResult {
    debug!("open folder by using year with ordinal day: {day_of_year:?}");
    let wanted_daily = DailyName::create_from_ordinal_day(day_of_year)?;

    open_daily_with_editor(&wanted_daily)
}
pub fn open_by_year_month_day(ymd: &DayMonthYear) -> AppResult {
    debug!("open folder by using year, month and day: {ymd:?}");
    let wanted_daily = DailyName::creaet_from_year_month_day(ymd)?;

    open_daily_with_editor(&wanted_daily)
}

fn open_daily_with_editor(wanted_daily: &DailyName) -> AppResult {
    let to_open = file_access::create_new_path_for(wanted_daily.name())?;
    process_handling::start_process_with(&to_open)?;

    Ok(())
}
