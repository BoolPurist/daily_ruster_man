use super::{daily_filtering, DailyName};
use crate::prelude::*;
use dirs;
use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};

pub fn fetch_valid_daily_entries() -> AppResult<Vec<DailyName>> {
    let daily_paths = get_all_daily_paths()?;

    let file_names = daily_filtering::strip_expect_file_name(&daily_paths);

    let filtered = daily_filtering::filter_out_non_daily(file_names).collect();

    Ok(filtered)
}

pub fn create_new_path_for(file_name: &str) -> AppResult<PathBuf> {
    let data_folder_root = get_and_ensure_path_to_daily()?;

    Ok(data_folder_root.join(file_name))
}

pub fn get_all_daily_paths() -> AppResult<Vec<PathBuf>> {
    let data_folder = get_and_ensure_path_to_daily()?;

    let gathered_dailies = fs::read_dir(&data_folder)
        .map_err(AppError::new)?
        .filter_map(|entry| match entry {
            Ok(resolved) => {
                if is_file(&resolved) {
                    Some(resolved.path())
                } else {
                    None
                }
            }
            Err(error) => {
                warn!(
                    "Entry could not be read in directory {0:?}\n. Cause: {error}",
                    &data_folder
                );
                None
            }
        })
        .collect();

    return Ok(gathered_dailies);

    fn is_file(to_check: &DirEntry) -> bool {
        if let Ok(file_type) = to_check.file_type() {
            file_type.is_file()
        } else {
            false
        }
    }
}

/// # Summary
///
/// It provides the path where the dailies are stored.
/// This returned path is ensured to be created already in case of no error.
///
/// # Error
///
/// - If no path to the app data directory of user can be retrieved.
/// - If the path to the app data directory was not created so far.
fn get_and_ensure_path_to_daily() -> AppResult<PathBuf> {
    let data_dir = fetch_paths_names::fetch_path_with_dailys()?;

    // Sure: with data_dir the existence of general data path is unsured.
    // Now we ensure that a folder within this existing data_dir named after this app is there.
    fs::create_dir_all(&data_dir)?;

    Ok(data_dir)
}

mod fetch_paths_names {

    use super::*;

    pub fn fetch_path_with_dailys() -> AppResult<PathBuf> {
        let app_name = Path::new(get_app_name());
        let data_folder = dirs::data_dir()
            .context("Could find a data folder for dailies under the current user")?;

        debug!("Using {data_folder:?} as general data folder");

        let exits = data_folder.try_exists()?;
        if !exits {
            bail!(
                "Designated path for app data {:?} does not exits",
                data_folder
            )
        }

        let app_data_folder = data_folder.join(app_name);
        debug!("Using {app_data_folder:?} as data folder for app.");

        Ok(app_data_folder)
    }

    pub fn get_app_name() -> &'static str {
        env!("CARGO_PKG_NAME")
    }
}
