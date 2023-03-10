use crate::core::date_models::find_by::FindByYearMonthDay;
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
        FindByYearMonthDay::new(Some(2000), Some(1), Some(1)).unwrap(),
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
        FindByYearMonthDay::new(Some(1999), None, None).unwrap(),
    );
    // // Should match serveral with given month
    // assert_filter_is_correct(
    //     vec![
    //         create_daily_name_from(2001, 8, 11),
    //         create_daily_name_from(2000, 1, 1),
    //         create_daily_name_from(1999, 1, 2),
    //         create_daily_name_from(1999, 2, 3),
    //         create_daily_name_from(1980, 4, 13),
    //     ],
    //     vec![
    //         create_daily_name_from(2000, 1, 1),
    //         create_daily_name_from(1999, 1, 2),
    //     ],
    //     FindByYearMonthDay::new(None, Some(1), None).unwrap(),
    // );
    // // Should match one with given day
    // assert_filter_is_correct(
    //     vec![
    //         create_daily_name_from(2001, 8, 11),
    //         create_daily_name_from(2000, 1, 1),
    //         create_daily_name_from(1999, 1, 2),
    //         create_daily_name_from(1999, 2, 3),
    //         create_daily_name_from(1980, 4, 13),
    //     ],
    //     vec![create_daily_name_from(2001, 8, 11)],
    //     FindByYearMonthDay::new(None, None, Some(11)).unwrap(),
    // );
    // // Should math with given month and year
    // assert_filter_is_correct(
    //     vec![
    //         create_daily_name_from(2001, 8, 11),
    //         create_daily_name_from(2000, 4, 1),
    //         create_daily_name_from(1999, 4, 2),
    //         create_daily_name_from(1999, 4, 23),
    //         create_daily_name_from(1980, 4, 13),
    //     ],
    //     vec![
    //         create_daily_name_from(1999, 4, 2),
    //         create_daily_name_from(1999, 4, 23),
    //     ],
    //     FindByYearMonthDay::new(Some(1999), Some(4), None).unwrap(),
    // );
    // // Should math with given month and day
    // assert_filter_is_correct(
    //     vec![
    //         create_daily_name_from(2001, 8, 11),
    //         create_daily_name_from(2000, 4, 1),
    //         create_daily_name_from(1999, 4, 2),
    //         create_daily_name_from(1999, 4, 12),
    //         create_daily_name_from(1980, 4, 12),
    //         create_daily_name_from(1978, 4, 12),
    //     ],
    //     vec![
    //         create_daily_name_from(1999, 4, 12),
    //         create_daily_name_from(1980, 4, 12),
    //         create_daily_name_from(1978, 4, 12),
    //     ],
    //     FindByYearMonthDay::new(None, Some(4), Some(12)).unwrap(),
    // );
    // // Should math with given year and day
    // assert_filter_is_correct(
    //     vec![
    //         create_daily_name_from(2000, 8, 11),
    //         create_daily_name_from(2000, 7, 1),
    //         create_daily_name_from(1999, 4, 2),
    //         create_daily_name_from(1999, 4, 12),
    //         create_daily_name_from(1980, 4, 12),
    //         create_daily_name_from(1978, 4, 12),
    //     ],
    //     vec![create_daily_name_from(2000, 7, 1)],
    //     FindByYearMonthDay::new(Some(2000), None, Some(1)).unwrap(),
    // );
}

fn assert_filter_is_correct(
    given: Vec<DailyName>,
    expected: Vec<DailyName>,
    filtering: FindByYearMonthDay,
) {
    let actual = filter_dailies_by_ymd(given, &filtering);
    assert_eq!(expected, actual);
}

fn create_daily_name_from(y: u32, m: u32, d: u32) -> DailyName {
    DailyName::new(y, m, d, "does_not_matter").expect("y, m, and d do lead to valid daily name")
}
