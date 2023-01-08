use super::file_access;

pub fn fetch_all_daily_names() -> Vec<String> {
    let mut filter_by_valid_format = file_access::fetch_valid_daily_entries();

    filter_by_valid_format.sort();
    filter_by_valid_format.reverse();

    filter_by_valid_format
        .into_iter()
        .map(|str| str.to_string())
        .collect()
}
