mod daily_names;
mod file_access;

use core::borrow::Borrow;
use std::path::Path;
use std::path::PathBuf;
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
    let daily_paths: Vec<PathBuf> = file_access::get_all_daily_paths().collect();
    let file_names = strip_expect_file_name(&daily_paths);
    let filter_by_valid_format = filter_out_non_daily(&file_names);

    filter_by_valid_format
        .into_iter()
        .map(|str| str.to_string())
        .collect()
}

fn filter_out_non_daily<'a, T>(to_filter: &[&'a T]) -> Vec<DailyName>
where
    T: AsRef<str> + ?Sized,
{
    to_filter
        .iter()
        .filter_map(|file_name| file_name.as_ref().parse().ok())
        .collect()
}

fn strip_expect_file_name<'a, T>(paths: &'a [T]) -> Vec<&'a str>
where
    T: AsRef<Path>,
{
    paths
        .iter()
        .map(|full_path| {
            full_path
                .as_ref()
                .file_name()
                .expect("Could not get file name")
                .to_str()
                .expect("Could convert os string to utf string")
        })
        .collect()
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
    use super::strip_expect_file_name;
    use std::path::Path;
    #[test]
    fn test_strip_expect_file_name() {
        let given = vec![
            Path::new("/home/bla/text.txt"),
            Path::new("/home/bla/aaa.a"),
            Path::new("/home/bla/ssss.s"),
        ];

        let expected = vec!["text.txt", "aaa.a", "ssss.s"];

        let actual = strip_expect_file_name(&given);
        assert_eq!(expected, actual);
    }
}
