use super::{
    date_models::open_by::{
        OpenByDayMonthYear, OpenByDayOfYear, OpenByDaysInTime, OpenByMonthInYear,
    },
    file_access, process_handling, DailyName,
};

use crate::{AppResult, core::dates_names::monthly_name::MonthlyName};
pub fn open_today() -> AppResult {
    let today_name = DailyName::create_today_name();

    open_daily_with_editor(today_name.name())
}

pub fn open_by_month_in_year(month_in_year: OpenByMonthInYear) -> AppResult {
    debug!("open month in year by looking at {month_in_year:?}");

    let wanted_month = MonthlyName::from_month_in_year(&month_in_year)?;
    open_daily_with_editor(wanted_month.name())
}

pub fn open_by_future_past_range(range: &OpenByDaysInTime) -> AppResult {
    debug!("open day by looking at range {range:?}");
    let wanted_daily = DailyName::create_from_range(range);

    open_daily_with_editor(wanted_daily.name())
}
pub fn open_by_day_of_year(day_of_year: &OpenByDayOfYear) -> AppResult {
    debug!("open dae by using year with ordinal day: {day_of_year:?}");
    let wanted_daily = DailyName::create_from_ordinal_day(day_of_year)?;

    open_daily_with_editor(wanted_daily.name())
}
pub fn open_by_year_month_day(ymd: &OpenByDayMonthYear) -> AppResult {
    debug!("open day by using year, month and day: {ymd:?}");
    let wanted_daily = DailyName::create_from_year_month_day(ymd)?;

    open_daily_with_editor(wanted_daily.name())
}

fn open_daily_with_editor(name_journal: &str) -> AppResult {
    let to_open = file_access::create_new_path_for(name_journal)?;
    process_handling::start_process_with(&to_open)?;

    Ok(())
}
