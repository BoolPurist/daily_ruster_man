mod daily_names;
mod file_access;

use std::path::Path;
use std::process::Command;

use self::daily_names::DailyName;

const NVIM: &str = "nvim";
pub fn open_today() {
    let today_name = create_today_name();
    let to_open = file_access::create_new_path_for(today_name.get_name());

    start_process_with(&to_open);
}

fn create_today_name() -> DailyName {
    let now = chrono::Local::now();
    let date_now = now.date_naive();

    DailyName::new(date_now, daily_names::MD_EXT)
}

pub fn fetch_all_daily_names() -> Vec<String> {
    let daily_paths = file_access::get_all_daily_paths();

    let file_names = strip_expect_file_name(&daily_paths);
    let filter_by_valid_format = filter_out_non_daily(file_names);

    filter_by_valid_format.map(|str| str.to_string()).collect()
}

/// TODO: write unit tests for this func
fn filter_out_non_daily<'a>(
    to_filter: impl Iterator<Item = &'a str> + 'a,
) -> impl Iterator<Item = DailyName> + 'a {
    to_filter.filter_map(|file_name| file_name.parse().ok())
}

fn strip_expect_file_name<T>(paths: &[T]) -> impl Iterator<Item = &str>
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

fn start_process_with(path: &Path) {
    let path_as_str = path
        .to_str()
        .expect("Could not convert path to a text as argument for editor.");

    Command::new(NVIM)
        .arg(path_as_str)
        .spawn()
        .expect("Could not spawn editor as child process")
        .wait()
        .expect("editor failed");
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
            .map(|daily_name| daily_name.get_date())
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
