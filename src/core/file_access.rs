use std::{
    str::FromStr,
    path::{Path, PathBuf},
    fs::{self, DirEntry},
};
use crate::core::app_options::AppOptions;
use dirs;
use crate::prelude::*;
use super::date_filtering;

pub fn resolve_str_as_path(to_resolve: &str) -> PathBuf {
    let expanded_path = shellexpand::full_with_context_no_errors(
        to_resolve,
        || dirs::home_dir().map(|home_dir| home_dir.to_string_lossy().to_string()),
        |env_var| std::env::var(env_var).ok(),
    );

    let path = PathBuf::from(expanded_path.as_ref());

    match fs::canonicalize(&path) {
        Ok(absolute) => absolute,
        Err(_) => {
            warn!("Path {:?} can not be resolved to absolute path. So it will be used as not a absolute path", &path);
            path
        }
    }
}

pub fn resolve_path(to_resolve: &Path) -> PathBuf {
    let as_string = to_resolve.to_string_lossy();
    resolve_str_as_path(&as_string)
}

pub fn fetch_data_path(option: &AppOptions) -> AppResult<PathBuf> {
    let app_data_folder = if option.use_prod_local_share() {
        fetch_paths_names::fetch_prod_data_dir()
    } else {
        fetch_paths_names::fetch_dev_data_dir()
    }?;

    debug!("Using {app_data_folder:?} as data folder for app.");

    Ok(app_data_folder)
}

pub fn fetch_path_conf(option: &AppOptions) -> AppResult<PathBuf> {
    let conf_dir = if option.use_prod_local_share() {
        fetch_paths_names::fetch_prod_conf_dir()
    } else {
        fetch_paths_names::fetch_dev_conf_dir()
    }?;

    Ok(conf_dir)
}

pub fn fetch_valid_date_entries<R>(option: &AppOptions) -> AppResult<Vec<R>>
where
    R: FromStr,
{
    let file_names = fetch_file_names_from_dates(option)?;

    let filtered = file_names
        .into_iter()
        .filter_map(|file_name| file_name.parse().ok())
        .collect();

    Ok(filtered)
}
pub fn create_new_path_for(file_name: &str, option: &AppOptions) -> AppResult<PathBuf> {
    let data_folder_root = option.get_data_path()?;

    Ok(data_folder_root.join(file_name))
}

pub fn get_all_journal_paths(option: &AppOptions) -> AppResult<Vec<PathBuf>> {
    let data_folder = option
        .get_data_path()
        .context("Failed to get path to data/journals")?;

    let gathered_dailies = fs::read_dir(&data_folder)
        .map_err(AppError::new)
        .with_context(|| {
            format!(
                "At path {:?} there is no directory which can contain journals to find",
                &data_folder
            )
        })?
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

fn fetch_file_names_from_dates(option: &AppOptions) -> AppResult<Vec<String>> {
    let journal_paths = get_all_journal_paths(option)?;

    let file_names = date_filtering::strip_expect_file_name(&journal_paths);

    Ok(file_names.map(|slice| slice.to_owned()).collect())
}

mod fetch_paths_names {

    use super::*;
    use crate::core::constants::{DEV_DATA_FOLDER, DEV_CONF_FOLDER};

    pub fn fetch_dev_data_dir() -> AppResult<PathBuf> {
        fetch_dev_dir_for(&DEV_DATA_FOLDER)
    }

    pub fn fetch_dev_conf_dir() -> AppResult<PathBuf> {
        fetch_dev_dir_for(&DEV_CONF_FOLDER)
    }

    fn fetch_dev_dir_for(dir: &Path) -> AppResult<PathBuf> {
        let project_root = get_project_folder()?;

        let dev_data_folder = project_root.join(dir);

        Ok(dev_data_folder)
    }

    fn fetch_some_prod_app_dir(
        dir_access: impl Fn() -> AppResult<PathBuf>,
        on_error_existing_check: impl Fn(&Path) -> String,
    ) -> AppResult<PathBuf> {
        let app_name = Path::new(get_app_name());
        let app_folder = dir_access()?;

        check_if_dir_exits(&app_folder, on_error_existing_check)?;

        // We have checked the generall data, conf, et cetra exits
        // Makre sure the folder with app name exits.
        let app_folder = app_folder.join(app_name);
        fs::create_dir_all(&app_folder)?;

        Ok(app_folder)
    }

    pub fn fetch_prod_data_dir() -> AppResult<PathBuf> {
        fetch_some_prod_app_dir(
            || {
                dirs::data_dir().ok_or_else(|| {
                    anyhow!("Could find a data folder for dailies under the current user")
                })
            },
            |data_folder| {
                format!(
                    "Designated path for app data {:?} does not exits",
                    data_folder
                )
            },
        )
    }
    pub fn fetch_prod_conf_dir() -> AppResult<PathBuf> {
        fetch_some_prod_app_dir(
            || {
                dirs::config_dir().ok_or_else(|| {
                    anyhow!("Could find a conf folder for dailies under the current user",)
                })
            },
            |conf_folder| {
                format!(
                    "Designated path for app conf {:?} does not exits",
                    conf_folder
                )
            },
        )
    }

    fn check_if_dir_exits(folder: &Path, on_error_exits: impl Fn(&Path) -> String) -> AppResult {
        let exits = folder.try_exists()?;

        if !exits {
            bail!(on_error_exits(folder))
        }

        Ok(())
    }

    fn get_project_folder() -> AppResult<PathBuf> {
        project_root::get_project_root()
            .map_err(AppError::new)
            .context("Could get the project folder from rust project")
    }

    pub fn get_app_name() -> &'static str {
        env!("CARGO_PKG_NAME")
    }
}
