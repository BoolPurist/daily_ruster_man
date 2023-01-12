use crate::{
    core::{
        dates_names::{MonthlyName, DailyName, ToDateTuple},
        date_models::find_by::{FindByYearMonthDay, FindByMonthInYear},
    },
    prelude::*,
};

use super::{file_access, dates_names::yearly_name::YearlyName};
use super::date_filtering;

pub fn fetch_all_daily_names(ymd_listing: &FindByYearMonthDay) -> AppResult<Vec<String>> {
    let with_valid_format: Vec<DailyName> = file_access::fetch_valid_date_entries()?;
    let filtered_by_ymd = date_filtering::filter_dailies_by_ymd(with_valid_format, ymd_listing);

    let sorted_date_tuple = sort_and_to_string(filtered_by_ymd);
    Ok(sorted_date_tuple)
}
pub fn fetch_all_monthly_names(month_in_year: &FindByMonthInYear) -> AppResult<Vec<String>> {
    let with_valid_format: Vec<MonthlyName> = file_access::fetch_valid_date_entries()?;

    let filtered_monthlies = date_filtering::filter_monthly_by_ym(with_valid_format, month_in_year);

    let sorted_compact_tuple = sort_and_to_string(filtered_monthlies);

    Ok(sorted_compact_tuple)
}
pub fn fetch_yearly_names() -> AppResult<Vec<String>> {
    let with_valid_format: Vec<YearlyName> = file_access::fetch_valid_date_entries()?;

    let sorted_compact_tuple = sort_and_to_string(with_valid_format);

    Ok(sorted_compact_tuple)
}

fn sort_and_to_string<T>(mut seq: Vec<T>) -> Vec<String>
where
    T: Ord + ToDateTuple,
{
    seq.sort();
    seq.reverse();

    seq.into_iter().map(|str| str.to_date_tuple()).collect()
}
