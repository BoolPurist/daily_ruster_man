pub mod command_processor;
use std::{borrow::Cow, collections::HashMap};
pub use command_processor::{CommandToExecute, OsCommandProcossor};
use regex::{Regex, Replacer};
use self::command_processor::CommandProcessor;

use super::app_config::AppConfig;

#[derive(Debug)]
pub enum PlaceholderTemplate<'a, T> {
    DirectValue(&'a str),
    Commmand(CommandToExecute<'a, T>),
}
pub fn replace_template_placeholders<'m, 't, T>(
    template: &'t str,
    placeholders: &'m mut HashMap<&'t str, PlaceholderTemplate<'t, T>>,
) -> Cow<'t, str>
where
    T: CommandProcessor,
{
    lazy_static! {
        static ref REGEX_PLACE_HOLDERS: Regex = Regex::new(r"(?mi)\{\s*(\S+?)\s*\}")
            .unwrap_or_else(|error| panic!(
                "Invalid expression inside function: {}\n. Reason: {:?}",
                stringify!(replace_template_placeholders),
                error
            ));
    }

    let mut found_errors_for_commmand: HashMap<String, String> = HashMap::new();
    let replacer = PlaceHolderReplacer::new(placeholders, &mut found_errors_for_commmand);

    let replacement = REGEX_PLACE_HOLDERS.replace_all(&template, replacer);

    for error in found_errors_for_commmand.into_iter() {
        error!(
            "For key {0} the command could be executed without errors.\n Error: {1}",
            error.0, error.1,
        )
    }
    replacement
}

pub struct PlaceHolderReplacer<'m, 'kv, T>
where
    T: CommandProcessor,
{
    map: &'m mut HashMap<&'kv str, PlaceholderTemplate<'kv, T>>,
    errors: &'m mut HashMap<String, String>,
}

impl<'m, 'kv, T> PlaceHolderReplacer<'m, 'kv, T>
where
    T: CommandProcessor,
{
    pub fn new(
        map: &'m mut HashMap<&'kv str, PlaceholderTemplate<'kv, T>>,
        errors: &'m mut HashMap<String, String>,
    ) -> Self {
        Self { map, errors }
    }
}

impl<'m, 'kv, T> Replacer for PlaceHolderReplacer<'m, 'kv, T>
where
    T: CommandProcessor,
{
    fn replace_append(&mut self, caps: &regex::Captures<'_>, dst: &mut String) {
        let full_match = caps
            .get(0)
            .expect("Unexpected: no full caputure group found even with a found match");
        let inner_match = caps.get(1).expect(
            "Unexpexted: no innner capture group found for matching placholders in a template",
        );

        match self.map.get_mut(inner_match.as_str()) {
            Some(to_insert) => match to_insert {
                PlaceholderTemplate::DirectValue(direct_insert) => {
                    dst.push_str(&direct_insert);
                }
                PlaceholderTemplate::Commmand(command_to_insert) => {
                    let command_output = command_to_insert.get_std_out();
                    dst.push_str(command_output);
                    let command_error = command_to_insert.get_std_err();
                    if command_error.is_empty() {
                        let key = inner_match.as_str();
                        if self.errors.contains_key(key) {
                            self.errors.insert(key.to_owned(), command_error.to_owned());
                        }
                    } else {
                    }
                }
            },
            None => dst.push_str(full_match.as_str()),
        }
    }
}

#[cfg(test)]
mod testing {
    use std::collections::HashMap;
    use super::{replace_template_placeholders, CommandToExecute};
    use crate::core::template::command_processor::testing::return_dummy_processed_command;
    use crate::core::template::PlaceholderTemplate;

    use mockall::*;
    use super::command_processor::MockCommandProcessor;
    type FakeCommandOutput<'l> = PlaceholderTemplate<'l, MockCommandProcessor>;
    fn create_dummmy_command_processor<'a>(
        command_text: &'a str,
        expected_error: &'a str,
        times: usize,
    ) -> FakeCommandOutput<'a> {
        let mut mock = MockCommandProcessor::new();
        let expected_error = expected_error.to_owned();
        mock.expect_process()
            .times(times)
            .returning(move |to_process| {
                (
                    return_dummy_processed_command(to_process),
                    expected_error.to_owned(),
                )
            });

        PlaceholderTemplate::Commmand(CommandToExecute::new_with(command_text, mock))
    }
    #[test]
    fn should_complement_template_with_command_processor() {
        let given_templa = r#"
# for {hello}
## Uptime is {how_long}
- Uptime is now {how_long_now}
## Uptime is {how_long} again
more {hello}
### Not {found}

Should inserted {how_long_error} even with errors
"#;
        let mut map: HashMap<&str, FakeCommandOutput> = HashMap::from_iter(
            vec![
                ("hello", PlaceholderTemplate::DirectValue("world")),
                ("how_long", create_dummmy_command_processor("uptime", "", 1)),
                (
                    "not_to_be_inserted",
                    create_dummmy_command_processor("oh_oh", "", 0),
                ),
                (
                    "how_long_error",
                    create_dummmy_command_processor("uptime xxx", "error", 1),
                ),
                (
                    "how_long_now",
                    create_dummmy_command_processor("uptime now", "", 1),
                ),
            ]
            .into_iter(),
        );

        // Act
        let actual = replace_template_placeholders(given_templa, &mut map);
        insta::assert_display_snapshot!(actual);
    }
}