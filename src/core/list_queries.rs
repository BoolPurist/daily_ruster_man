use super::file_access;

pub fn fetch_all_daily_names() -> Vec<String> {
    let filter_by_valid_format = file_access::fetch_valid_daily_entries();
    filter_by_valid_format
        .into_iter()
        .map(|str| str.to_string())
        .collect()
}
