pub mod path_from_config;
pub use path_from_config::PatchFromConfig;

use std::{
    path::{PathBuf, Path},
    collections::HashMap,
    borrow::Cow,
};

use serde::Deserialize;

use crate::core::template::{CommandToExecute, OsCommandProcossor};
use crate::prelude::*;

use super::{
    constants::CONF_FILE_NAME, template::PlaceholderTemplate, file_access, app_options::AppOptions,
    dates_names::ResolvePlaceholders,
};

macro_rules! path_from_conf_getter {
    ($field:ident) => {
        pub fn $field(&self) -> PatchFromConfig {
            PatchFromConfig::new(self.$field.clone())
        }
    };
}

#[derive(Deserialize, Default, Debug, Getters)]
/// Contains access to data which is provided by the conf file of app located at the app conf
/// folder
pub struct AppConfig {
    yearly_template: Option<String>,
    monthly_template: Option<String>,
    daily_template: Option<String>,
    #[getset(get = "pub")]
    data_foler: Option<String>,
    placeholders: Option<Vec<PlaceHolder>>,
    #[getset(get = "pub")]
    editor: Option<String>,
    #[serde(skip)]
    #[getset(get = "pub")]
    /// Path to folder where the config file loaded from
    root_path: PathBuf,
}

impl AppConfig {
    path_from_conf_getter! {monthly_template}
    path_from_conf_getter! {yearly_template}
    path_from_conf_getter! {daily_template}

    /// Fetches all placeholders from the config.
    pub fn create_template_placeholder_for<'a>(
        &'a self,
        journal: &impl ResolvePlaceholders,
    ) -> HashMap<&'_ str, PlaceholderTemplate<'_, OsCommandProcossor>> {
        return match &self.placeholders {
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
                                resolve_direct_value(to_convert, journal)
                            }
                        }
                        None => resolve_direct_value(to_convert, journal),
                    };
                    output.insert(to_convert.key().as_str(), value);
                }

                output
            }
        };

        fn resolve_direct_value<'a>(
            to_convert: &'a PlaceHolder,
            journal: &impl ResolvePlaceholders,
        ) -> PlaceholderTemplate<'a, OsCommandProcossor> {
            use crate::core::constants::{PREFIX_FOR_BUITLIN_VAR, SUFFIX_FOR_BUITLIN_VAR};

            let to_convert = to_convert.value().as_str();
            let trimmed = to_convert
                .trim_start_matches(PREFIX_FOR_BUITLIN_VAR)
                .trim_end_matches(SUFFIX_FOR_BUITLIN_VAR);

            let resolved = journal.resolve_variable(trimmed);

            // PREFIX_FOR_BUITLIN_VAR, left, and SUFFIX_FOR_BUITLIN_VAR, right was removed.
            // Use the intial ref with left and right not trimmed away
            if let Cow::Borrowed(_trimmed) = resolved {
                PlaceholderTemplate::DirectValue(to_convert.into())
            } else {
                PlaceholderTemplate::DirectValue(resolved)
            }
        }
    }

    pub fn try_from_file_system(option: &AppOptions) -> AppResult<Option<Self>> {
        let resolved_path = if let Some(path) = option.general().config_path() {
            debug!("Using  conf path provided by cli or env.");
            file_access::resolve_str_as_path(path)
        } else {
            debug!("Using conf path provided by os.");
            let path = file_access::fetch_path_conf(option)?;
            file_access::resolve_path(&path)
        };
        debug!("Using {:?} as conf folder for app.", &resolved_path);

        let path_to_conf_file = resolved_path.join(CONF_FILE_NAME);

        debug!("Using {:?} as conf file for app.", &path_to_conf_file);
        if path_to_conf_file.exists() {
            let content = std::fs::read_to_string(&path_to_conf_file)
                .context("could not read config file by given path")?;

            match toml::from_str::<AppConfig>(&content) {
                Ok(mut parsed_content) => {
                    parsed_content.root_path = resolved_path;
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

    /// Returns none if there is no template file at the given parameter.
    pub fn try_get_template_file_content(&self, path: &Path) -> AppResult<Option<String>> {
        if path.exists() {
            let template_content = std::fs::read_to_string(path)?;
            info!("Template path found at {:?}", path);
            Ok(Some(template_content))
        } else {
            info!("No template found at {:?}", path);
            Ok(None)
        }
    }
}

#[derive(Deserialize, Getters, Debug)]
#[getset(get = "pub")]
/// Represents a given placeholder from the user to augment a template with costum values or output
/// of a given command.
/// Placeholder are currrenlty retrieved from the config file.
pub struct PlaceHolder {
    /// which is searched for in the template
    key: String,
    /// by which the key in the template is replaced
    value: String,
    /// If true instead treat `value` as command to execute and use its output in the template
    is_command: Option<bool>,
}

#[cfg(test)]
mod testing {
    use crate::core::dates_names::{DailyName, MonthlyName, yearly_name::YearlyName};

    use super::*;
    #[test]
    fn should_resolve_placeholders_with_daily() {
        const TEST_INPUT: &str = r#"
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
[[placeholders]]
key = "day"
value = "{{DAY_JOURNAL}}"
[[placeholders]]
key = "month"
value = "{{MONTH_JOURNAL}}"
[[placeholders]]
key = "year"
value = "{{YEAR_JOURNAL}}"
"#;
        // Set up
        let config: AppConfig = toml::from_str(TEST_INPUT).expect("Invalid input from test input");

        let daily = DailyName::new(2000, 5, 15, crate::core::constants::MD_EXT).unwrap();
        let actual = act(&config, &daily);

        // Assert
        insta::assert_debug_snapshot!(actual);
    }

    #[test]
    fn should_resolve_placeholders_with_monthly() {
        const TEST_INPUT: &str = r#"
value = "{{DAY_JOURNAL}}"
[[placeholders]]
key = "month"
value = "{{MONTH_JOURNAL}}"
[[placeholders]]
key = "year"
value = "{{YEAR_JOURNAL}}"
"#;
        // Set up
        let config: AppConfig = toml::from_str(TEST_INPUT).expect("Invalid input from test input");

        let monthly = MonthlyName::from_ym(1980, 2, crate::core::constants::MD_EXT).unwrap();
        let actual = act(&config, &monthly);

        // Assert
        insta::assert_debug_snapshot!(actual);
    }
    #[test]
    fn should_resolve_placeholders_with_yearly() {
        const TEST_INPUT: &str = r#"
value = "{{DAY_JOURNAL}}"
[[placeholders]]
key = "month"
value = "{{MONTH_JOURNAL}}"
[[placeholders]]
key = "year"
value = "{{YEAR_JOURNAL}}"
"#;
        // Set up
        let config: AppConfig = toml::from_str(TEST_INPUT).expect("Invalid input from test input");

        let monthly = YearlyName::new(412.try_into().unwrap());
        let actual = act(&config, &monthly);

        // Assert
        insta::assert_debug_snapshot!(actual);
    }

    fn act<'a>(
        config: &'a AppConfig,
        journal: &'a impl ResolvePlaceholders,
    ) -> Vec<(&'a str, PlaceholderTemplate<'a, OsCommandProcossor>)> {
        let actual = config.create_template_placeholder_for(journal);

        // Prepare for assert
        let mut actual_as_vec: Vec<(&str, PlaceholderTemplate<'_, OsCommandProcossor>)> =
            actual.into_iter().collect();
        actual_as_vec.sort_by_key(|key_value| key_value.0);
        actual_as_vec
    }
}
