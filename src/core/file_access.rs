use std::{
    str::FromStr,
    path::{Path, PathBuf},
    fs::{self, DirEntry},
};

use dirs;

use crate::prelude::*;

use super::date_filtering;

pub fn fetch_valid_date_entries<R>() -> AppResult<Vec<R>>
where
    R: FromStr,
{
    let file_names = fetch_file_names_from_dates()?;

    let filtered = file_names
        .into_iter()
        .filter_map(|file_name| file_name.parse().ok())
        .collect();

    Ok(filtered)
}

pub fn create_new_path_for(file_name: &str) -> AppResult<PathBuf> {
    let data_folder_root = get_and_ensure_path_to_daily()?;

    Ok(data_folder_root.join(file_name))
}

pub fn get_all_journal_paths() -> AppResult<Vec<PathBuf>> {
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

fn fetch_file_names_from_dates() -> AppResult<Vec<String>> {
    let journal_paths = get_all_journal_paths()?;

    let file_names = date_filtering::strip_expect_file_name(&journal_paths);

    Ok(file_names.map(|slice| slice.to_owned()).collect())
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

    use project_root::get_project_root;

    use super::*;

    fn fetch_dev_data_folder_dir() -> AppResult<PathBuf> {
        let project_root = get_project_folder()?;

        let dev_data_folder = project_root.join(crate::core::constants::DEV_DATA_FOLDER.clone());

        Ok(dev_data_folder)
    }

    fn fetch_prod_data_dir() -> AppResult<PathBuf> {
        let app_name = Path::new(get_app_name());
        let data_folder = dirs::data_dir().ok_or_else(|| {
            anyhow!("Could find a data folder for dailies under the current user")
        })?;

        check_if_data_folder_exits(&data_folder)?;
        let app_data_folder = data_folder.join(app_name);

        Ok(app_data_folder)
    }

    fn check_if_data_folder_exits(data_folder: &Path) -> AppResult {
        let exits = data_folder.try_exists()?;
        if !exits {
            bail!(
                "Designated path for app data {:?} does not exits",
                data_folder
            )
        }

        Ok(())
    }

    pub fn fetch_path_with_dailys() -> AppResult<PathBuf> {
        let app_data_folder = if cfg!(debug_assertions) {
            fetch_dev_data_folder_dir()
        } else {
            fetch_prod_data_dir()
        }?;

        debug!("Using {app_data_folder:?} as data folder for app.");

        Ok(app_data_folder)
    }
    pub fn get_project_folder() -> AppResult<PathBuf> {
        get_project_root()
            .map_err(AppError::new)
            .context("Could get the project folder from rust project")
    }

    pub fn get_app_name() -> &'static str {
        env!("CARGO_PKG_NAME")
    }
}
