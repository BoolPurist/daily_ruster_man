use super::DailyName;
use std::path::Path;

pub fn filter_out_non_daily<'a>(
    to_filter: impl Iterator<Item = &'a str> + 'a,
) -> impl Iterator<Item = DailyName> + 'a {
    to_filter.filter_map(|file_name| file_name.parse().ok())
}

pub fn strip_expect_file_name<T>(paths: &[T]) -> impl Iterator<Item = &str>
where
    T: AsRef<Path>,
{
    paths.iter().map(|full_path| {
        full_path
            .as_ref()
            .file_name()
            .expect("Could not get file name")
            .to_str()
            .expect("Could convert os string to utf string")
    })
}

#[cfg(test)]
mod testing {
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
}
