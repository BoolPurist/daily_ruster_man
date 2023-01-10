use crate::core::{dates_names::daily_names::DailyName, date_models::FilterParamsYmD};
use std::path::Path;

#[cfg(test)]
#[path = "testing/test_daily_filtering.rs"]
mod testing;

pub fn filter_out_non_daily<'a>(
    to_filter: impl Iterator<Item = &'a str> + 'a,
) -> impl Iterator<Item = DailyName> + 'a {
    to_filter.filter_map(|file_name| file_name.parse().ok())
}
/// # Errors
/// - If path does not end with file name
/// - If file_name is not valid utf8
pub fn strip_expect_file_name<T>(paths: &[T]) -> impl Iterator<Item = &str>
where
    T: AsRef<Path>,
{
    let file_names_iter = paths.iter().filter_map(|full_path| {
        let full_path_ref = full_path.as_ref();
        let file_name = match full_path_ref.file_name() {
            Some(name) => name,
            None => {
                warn!(
                    "Could not get file name from path {full_path_ref:?} because it might end .. or is not file"
                );
                return None;
            }
        };

        match file_name.to_str() {
            Some(valid) => Some(valid),
            None => {
                warn!("Given file name {file_name:?} is not valid utf8");
                None
            }
        }
    });

    file_names_iter
}

pub fn filter_dailies_by_ymd(
    to_filter: Vec<DailyName>,
    ymd_listing: &FilterParamsYmD,
) -> Vec<DailyName> {
    type FilterDate = (fn(&DailyName, u32) -> bool, u32);

    let mut filters: Vec<FilterDate> = Vec::with_capacity(3);

    if let Some(year) = ymd_listing.year() {
        filters.push((filter_by_year, year));
    }
    if let Some(month) = ymd_listing.month() {
        filters.push((filter_by_month, month));
    }
    if let Some(day) = ymd_listing.day() {
        filters.push((filter_by_day, day));
    }

    to_filter
        .into_iter()
        .filter(|next| {
            filters.iter().all(|next_filter| {
                let (fnc, comp) = next_filter;
                fnc(next, *comp)
            })
        })
        .collect()
}

fn filter_by_year(to_check: &DailyName, year: u32) -> bool {
    to_check.is_in_year(year)
}
fn filter_by_month(to_check: &DailyName, month: u32) -> bool {
    to_check.is_in_month(month)
}
fn filter_by_day(to_check: &DailyName, day: u32) -> bool {
    to_check.is_in_day(day)
}
