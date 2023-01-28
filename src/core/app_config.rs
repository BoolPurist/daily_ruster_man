use crate::core::placeholder::CommandToExecute;
use serde::Deserialize;
use crate::prelude::*;
use crate::core::placeholder::OsCommandProcossor;
use std::{path::PathBuf, collections::HashMap};

use super::{file_access, constants::CONF_FILE_NAME, placeholder::PlaceholderTemplate};

#[derive(Deserialize, Default, Debug)]
pub struct AppConfig {
    yearly_template: Option<String>,
    monthly_template: Option<String>,
    daily_template: Option<String>,
    placeholders: Option<Vec<PlaceHolder>>,
    #[serde(skip)]
    /// Path to folder where the config file loaded from
    root_path: PathBuf,
}

impl AppConfig {
    pub fn create_placeholder_for_template<'a>(
        &'a self,
    ) -> HashMap<&'_ str, PlaceholderTemplate<'_, OsCommandProcossor>> {
        match &self.placeholders {
            None => HashMap::new(),
            Some(read_placeholders_from_config) => {
                let mut output: HashMap<&str, PlaceholderTemplate<'a, OsCommandProcossor>> =
                    HashMap::with_capacity(read_placeholders_from_config.len());
                for to_convert in read_placeholders_from_config {
                    let value = match to_convert.is_command() {
                        Some(is) => {
                            if *is {
                                PlaceholderTemplate::Commmand(CommandToExecute::new(
                                    to_convert.value(),
                                ))
                            } else {
                                PlaceholderTemplate::DirectValue(to_convert.value().as_str())
                            }
                        }

                        None => PlaceholderTemplate::DirectValue(to_convert.value()),
                    };
                    output.insert(to_convert.key().as_str(), value);
                }

                output
            }
        }
    }
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

#[cfg(test)]
mod testing {
    use super::*;
    #[test]
    fn should_transform_to_empty() {
        const input: &str = r#"
[[placeholders]]
key = "hello"
value = "world"
[[placeholders]]
key = "no_command"
value = "pls_no_execute"
is_command=false 
[[placeholders]]
key = "command"
value = "echo hello"
is_command=true
"#;
        let config: AppConfig = toml::from_str(input).expect("Invalid input from test input");
        let actual = config.create_placeholder_for_template();
        let mut actual_as_vec: Vec<(&str, PlaceholderTemplate<'_, OsCommandProcossor>)> =
            actual.into_iter().collect();
        actual_as_vec.sort_by_key(|key_value| key_value.0);
        insta::assert_debug_snapshot!(actual_as_vec);
    }
}
