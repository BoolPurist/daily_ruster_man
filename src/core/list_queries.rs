use crate::prelude::*;
use super::file_access;
use super::{data_models::FilterParamsYmD, daily_filtering};

pub fn fetch_all_daily_names(ymd_listing: &FilterParamsYmD) -> AppResult<Vec<String>> {
    let with_valid_format = file_access::fetch_valid_daily_entries()?;
    let mut filtered_by_ymd =
        daily_filtering::filter_dailies_by_ymd(with_valid_format, ymd_listing);

    filtered_by_ymd.sort();
    filtered_by_ymd.reverse();

    let fetched = filtered_by_ymd
        .into_iter()
        .map(|str| str.to_ymd_tuple())
        .collect();

    Ok(fetched)
}
