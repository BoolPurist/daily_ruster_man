use super::{daily_filtering, DailyName};
use dirs;
use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};

pub fn fetch_valid_daily_entries() -> Vec<DailyName> {
    let daily_paths = get_all_daily_paths();

    let file_names = daily_filtering::strip_expect_file_name(&daily_paths);

    daily_filtering::filter_out_non_daily(file_names).collect()
}

pub fn create_new_path_for(file_name: &str) -> PathBuf {
    let data_folder_root = fetch_ensure_data_folder();

    data_folder_root.join(file_name)
}

pub fn get_all_daily_paths() -> Vec<PathBuf> {
    let data_folder = fetch_ensure_data_folder();

    return fs::read_dir(data_folder)
        .expect("Could not get data_folder")
        .filter_map(|entry| {
            let resolved = entry.expect("io error");
            if is_file(&resolved) {
                Some(resolved.path())
            } else {
                None
            }
        })
        .collect();

    fn is_file(to_check: &DirEntry) -> bool {
        if let Ok(file_type) = to_check.file_type() {
            file_type.is_file()
        } else {
            false
        }
    }
}

fn fetch_path_with_dailys() -> PathBuf {
    let app_name = Path::new(get_app_name());
    let data_folder = dirs::data_dir().expect("No data folder found");

    debug!("Using {data_folder:?} as data folder");

    data_folder.join(app_name)
}

fn fetch_ensure_data_folder() -> PathBuf {
    let data_folder = fetch_path_with_dailys();
    fs::create_dir_all(&data_folder).expect("Could not ensure creation of data folder.");

    data_folder
}

fn get_app_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}
