use chrono::NaiveDate;

use date_validation_types::ValidatedDate;
use crate::core::{date_models::open_by::OpenByDaysInTime, DailyName, constants::MD_EXT};
use super::{daily_names::ParseDailyNameError, ToDateTuple, DateNameForFile};

#[test]
fn test_parse_error() {
    assert_if_parse_fails_with("daily.md", ParseDailyNameError::MissingYear);
    assert_if_parse_fails_with("2000_daily.md", ParseDailyNameError::MissingMonth);
    assert_if_parse_fails_with("aa_02_07_daily.md", ParseDailyNameError::YearNotANumber);
    assert_if_parse_fails_with("2000_z_2_daily.md", ParseDailyNameError::MonthNotANumber);
    assert_if_parse_fails_with("2000_2_-7_daily.md", ParseDailyNameError::DayNotANumber);
    assert_if_parse_fails_with("2000_2_32_daily.md", ParseDailyNameError::InvalidDate);
    assert_if_parse_fails_with("2000_0_1_daily.md", ParseDailyNameError::InvalidDate);
    assert_if_parse_fails_with("-78_2_2_daily.md", ParseDailyNameError::YearNotANumber);
}

#[test]
fn should_turn_to_date_tuple() {
    assert_turn_to_date_tuple(2000, 7, 3, "2000 07 03");
    assert_turn_to_date_tuple(1980, 11, 23, "1980 11 23");
    assert_turn_to_date_tuple(2011, 4, 13, "2011 04 13");
    assert_turn_to_date_tuple(2023, 10, 6, "2023 10 06");
}

#[test]
fn test_parse() {
    assert_parse(
        "2023_01_07_daily.md",
        NaiveDate::from_ymd_opt(2023, 1, 7).unwrap(),
    );
    assert_parse(
        "1981_11_24_daily.md",
        NaiveDate::from_ymd_opt(1981, 11, 24).unwrap(),
    );
    assert_parse(
        "2001_02_20_daily.md",
        NaiveDate::from_ymd_opt(2001, 2, 20).unwrap(),
    );
}

#[test]
fn test_create_daily_in_past() {
    assert_daily_from_range(OpenByDaysInTime::Past(2), 2022, 2, 8, 2022, 2, 6);
    assert_daily_from_range(OpenByDaysInTime::Past(40), 2022, 2, 7, 2021, 12, 29);
    assert_daily_from_range(OpenByDaysInTime::Past(0), 1980, 1, 1, 1980, 1, 1);
}
#[test]
fn test_create_daily_in_future() {
    assert_daily_from_range(OpenByDaysInTime::Future(2), 2022, 2, 8, 2022, 2, 10);
    assert_daily_from_range(OpenByDaysInTime::Future(40), 2022, 2, 7, 2022, 3, 19);
    assert_daily_from_range(OpenByDaysInTime::Future(0), 1980, 1, 1, 1980, 1, 1);
}

#[test]
fn test_new() {
    assert_daily_name_new(2000, 8, 20);
    assert_daily_name_new(1988, 5, 2);
    assert_daily_name_new(2022, 1, 30);
    assert_daily_name_new(1970, 12, 1);
}

fn assert_turn_to_date_tuple(y: u32, m: u32, d: u32, expected: &str) {
    let given = DailyName::new(y, m, d, MD_EXT)
        .expect("Invalid date provided for tuple ")
        .to_date_tuple();

    assert_eq!(expected, given);
}

fn assert_daily_from_range(
    range: OpenByDaysInTime,
    g_year: u32,
    g_month: u32,
    g_day: u32,
    ex_year: u32,
    ex_month: u32,
    ex_day: u32,
) {
    let given_date: ValidatedDate = NaiveDate::from_ymd_opt(g_year as i32, g_month, g_day)
        .expect("Invalid given date")
        .into();
    let expected =
        DailyName::new(ex_year, ex_month, ex_day, MD_EXT).expect("Invalid expected date");

    let actual: ValidatedDate = range
        .from_point_in_time(given_date)
        .expect("Invalid past or future steps");
    let actual_daily: DailyName = actual.into();
    assert_eq!(expected, actual_daily);
}

/// # panics
/// - If param `y` is not a valid year.
/// - If `m` param is not a valid month.
/// - if `d` param is not a valid day of given `m` month.
fn assert_daily_name_new(y: u32, m: u32, d: u32) {
    let expected = format!("{y}_{m:02}_{d:02}_daily.{MD_EXT}");

    let actual = DailyName::new(y, m, d, MD_EXT).expect("Invalid date for daily name");
    assert_eq!(expected, *actual.name());
}

fn assert_if_parse_fails_with(invalid_input: &str, expected_error: ParseDailyNameError) {
    match invalid_input.parse::<DailyName>() {
        Ok(_) => panic!("Should not parse on this invalid input {}", invalid_input),
        Err(error) => assert_eq!(error, expected_error),
    };
}

fn assert_parse(valid: &str, expected: NaiveDate) {
    if let Ok(parsed) = valid.parse::<DailyName>() {
        assert_eq!(parsed.date(), expected.into());
    } else {
        panic!("Did parse valid input correctly, ({})", valid);
    }
}
