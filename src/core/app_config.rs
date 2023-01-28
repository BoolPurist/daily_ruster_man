use serde::Deserialize;
use crate::prelude::*;
use std::path::PathBuf;

use super::{file_access, constants::CONF_FILE_NAME};

#[derive(Deserialize)]
pub struct AppConfig {
    yearly_template: Option<String>,
    monthly_template: Option<String>,
    daily_template: Option<String>,
    // TODO: Implement place hodlers
    _placeholders: Option<Vec<PlaceHolder>>,
    #[serde(skip)]
    /// Path to folder where the config file loaded from
    root_path: PathBuf,
}

impl AppConfig {
    pub fn try_from_file_system() -> AppResult<Option<Self>> {
        let path_to_configs = file_access::fetch_path_conf()?;
        let path_to_conf_file = path_to_configs.join(CONF_FILE_NAME);

        if path_to_conf_file.exists() {
            let content = std::fs::read_to_string(&path_to_conf_file)?;

            match toml::from_str::<AppConfig>(&content) {
                Ok(mut parsed_content) => {
                    parsed_content.root_path = path_to_configs;
                    Ok(Some(parsed_content))
                }
                Err(error) => {
                    warn!("App config file is not in valid format.\n Error: {}", error);
                    Ok(None)
                }
            }
        } else {
            info!("No config file found at {:?}", path_to_conf_file);
            Ok(None)
        }
    }

    pub fn try_get_daily_template(&self) -> AppResult<Option<String>> {
        self.try_get_template_file_content(|conf| conf.daily_template.as_deref())
    }
    pub fn try_get_monthly_template(&self) -> AppResult<Option<String>> {
        self.try_get_template_file_content(|conf| conf.monthly_template.as_deref())
    }
    pub fn try_get_yearly_template(&self) -> AppResult<Option<String>> {
        self.try_get_template_file_content(|conf| conf.yearly_template.as_deref())
    }

    fn try_get_template_file_content(
        &self,
        on_get_template: impl Fn(&AppConfig) -> Option<&str>,
    ) -> AppResult<Option<String>> {
        if let Some(template_path) = on_get_template(self) {
            let path = self.root_path.join(template_path);
            if path.exists() {
                let template_content = std::fs::read_to_string(&path)?;
                info!("Template path found at {:?}", path);
                Ok(Some(template_content))
            } else {
                info!("No template found at {:?}", path);
                Ok(None)
            }
        } else {
            info!("No template path found in config");
            Ok(None)
        }
    }
}

#[derive(Deserialize, Getters, Debug)]
#[getset(get = "pub")]
pub struct PlaceHolder {
    key: String,
    value: String,
    is_command: Option<bool>,
}
