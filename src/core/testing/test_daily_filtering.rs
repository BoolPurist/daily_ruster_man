use chrono::NaiveDate;

use super::*;
use std::path::Path;
#[test]
fn test_strip_expect_file_name() {
    let given = vec![
        Path::new("/home/bla/text.txt"),
        Path::new("/home/bla/aaa.a"),
        Path::new("/home/bla/ssss.s"),
    ];

    let expected = vec!["text.txt", "aaa.a", "ssss.s"];

    let actual: Vec<&str> = strip_expect_file_name(&given).collect();

    assert_eq!(expected, actual);
}

#[test]
fn test_filter_out_non_daily() {
    let given = ["text.txt", "2022_02_2_daily.md", "2001_2_22_daily.md"];
    let actual: Vec<NaiveDate> = filter_out_non_daily(given.into_iter())
        .map(|daily_name| daily_name.date())
        .collect();

    assert_eq!(
        actual,
        vec![
            NaiveDate::from_ymd_opt(2022, 2, 2).unwrap(),
            NaiveDate::from_ymd_opt(2001, 2, 22).unwrap()
        ]
    );
}

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
