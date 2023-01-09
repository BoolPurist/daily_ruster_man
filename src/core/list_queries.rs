use super::file_access;
use super::{data_models::FilterParamsYmD, daily_names::DailyName};

pub fn fetch_all_daily_names(ymd_listing: &FilterParamsYmD) -> Vec<String> {
    let with_valid_format = file_access::fetch_valid_daily_entries();
    let mut filtered_by_ymd = filter_dailies_by_ymd(with_valid_format, ymd_listing);

    filtered_by_ymd.sort();
    filtered_by_ymd.reverse();

    filtered_by_ymd
        .into_iter()
        .map(|str| str.to_ymd_tuple())
        .collect()
}

fn filter_dailies_by_ymd(
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

#[cfg(test)]
mod testing {
    use chrono::NaiveDate;
    use super::*;

    #[test]
    fn should_filter_by_ymd() {
        // Should match exactly one date
        assert_filter_is_correct(
            vec![
                create_daily_name_from(2001, 11, 21),
                create_daily_name_from(2000, 1, 1),
                create_daily_name_from(2000, 2, 3),
                create_daily_name_from(1980, 2, 13),
            ],
            vec![create_daily_name_from(2000, 1, 1)],
            FilterParamsYmD::new(Some(2000), Some(1), Some(1)),
        );
        // Should match serveral with given year
        assert_filter_is_correct(
            vec![
                create_daily_name_from(2001, 8, 11),
                create_daily_name_from(2000, 1, 1),
                create_daily_name_from(1999, 1, 1),
                create_daily_name_from(1999, 2, 3),
                create_daily_name_from(1980, 4, 13),
            ],
            vec![
                create_daily_name_from(1999, 1, 1),
                create_daily_name_from(1999, 2, 3),
            ],
            FilterParamsYmD::new(Some(1999), None, None),
        );
        // Should match serveral with given month
        assert_filter_is_correct(
            vec![
                create_daily_name_from(2001, 8, 11),
                create_daily_name_from(2000, 1, 1),
                create_daily_name_from(1999, 1, 2),
                create_daily_name_from(1999, 2, 3),
                create_daily_name_from(1980, 4, 13),
            ],
            vec![
                create_daily_name_from(2000, 1, 1),
                create_daily_name_from(1999, 1, 2),
            ],
            FilterParamsYmD::new(None, Some(1), None),
        );
        // Should match one with given day
        assert_filter_is_correct(
            vec![
                create_daily_name_from(2001, 8, 11),
                create_daily_name_from(2000, 1, 1),
                create_daily_name_from(1999, 1, 2),
                create_daily_name_from(1999, 2, 3),
                create_daily_name_from(1980, 4, 13),
            ],
            vec![create_daily_name_from(2001, 8, 11)],
            FilterParamsYmD::new(None, None, Some(11)),
        );
        // Should math with given month and year
        assert_filter_is_correct(
            vec![
                create_daily_name_from(2001, 8, 11),
                create_daily_name_from(2000, 4, 1),
                create_daily_name_from(1999, 4, 2),
                create_daily_name_from(1999, 4, 23),
                create_daily_name_from(1980, 4, 13),
            ],
            vec![
                create_daily_name_from(1999, 4, 2),
                create_daily_name_from(1999, 4, 23),
            ],
            FilterParamsYmD::new(Some(1999), Some(4), None),
        );
        // Should math with given month and day
        assert_filter_is_correct(
            vec![
                create_daily_name_from(2001, 8, 11),
                create_daily_name_from(2000, 4, 1),
                create_daily_name_from(1999, 4, 2),
                create_daily_name_from(1999, 4, 12),
                create_daily_name_from(1980, 4, 12),
                create_daily_name_from(1978, 4, 12),
            ],
            vec![
                create_daily_name_from(1999, 4, 12),
                create_daily_name_from(1980, 4, 12),
                create_daily_name_from(1978, 4, 12),
            ],
            FilterParamsYmD::new(None, Some(4), Some(12)),
        );
        // Should math with given year and day
        assert_filter_is_correct(
            vec![
                create_daily_name_from(2000, 8, 11),
                create_daily_name_from(2000, 7, 1),
                create_daily_name_from(1999, 4, 2),
                create_daily_name_from(1999, 4, 12),
                create_daily_name_from(1980, 4, 12),
                create_daily_name_from(1978, 4, 12),
            ],
            vec![create_daily_name_from(2000, 7, 1)],
            FilterParamsYmD::new(Some(2000), None, Some(1)),
        );
    }

    fn assert_filter_is_correct(
        given: Vec<DailyName>,
        expected: Vec<DailyName>,
        filtering: FilterParamsYmD,
    ) {
        let actual = filter_dailies_by_ymd(given, &filtering);
        assert_eq!(expected, actual);
    }

    fn create_daily_name_from(y: u32, m: u32, d: u32) -> DailyName {
        DailyName::new(
            NaiveDate::from_ymd_opt(y as i32, m, d).expect("Invalid date format for NaiveDate"),
            "does_not_matter",
        )
    }
}
