pub mod command_processor;
pub mod place_holder_replacer;

pub use command_processor::{CommandToExecute, OsCommandProcossor};

use std::{borrow::Cow, collections::HashMap};

use derive_new::new;

use place_holder_replacer::PlaceHolderReplacer;
use self::command_processor::CommandProcessor;
use crate::prelude::*;

#[derive(Debug)]
pub enum PlaceholderTemplate<'a, T> {
    DirectValue(&'a str),
    Commmand(CommandToExecute<'a, T>),
}

#[derive(Debug, new, Getters)]
pub struct TemplateReplacement<'t> {
    replacement: Cow<'t, str>,
    #[getset(get = "pub")]
    errors: HashMap<String, String>,
}

impl<'t> TemplateReplacement<'t> {
    pub fn replacement(&self) -> &str {
        &self.replacement
    }
}

pub fn replace_template_placeholders<'m, 't, T>(
    template: &'t str,
    placeholders: &'m mut HashMap<&'t str, PlaceholderTemplate<'t, T>>,
) -> TemplateReplacement<'m>
where
    T: CommandProcessor,
{
    let regex_place_holders = crate::regex! {r"(?mi)\{\s*(\S+?)\s*\}"};

    let mut found_errors_for_commmand: HashMap<String, String> = HashMap::new();
    let replacer = PlaceHolderReplacer::new(placeholders, &mut found_errors_for_commmand);

    let replacement = regex_place_holders.replace_all(template, replacer);

    TemplateReplacement::new(replacement, found_errors_for_commmand)
}

fn parse_commmand_text(to_parse: &str) -> AppResult<Vec<String>> {
    shellwords::split(to_parse)
        .map_err(AppError::new)
        .context("Missing closing quote for an argument of a given command.")
}

#[cfg(test)]
pub fn return_dummy_processed_command(input: &str) -> String {
    let splitted = parse_commmand_text(input).expect("Invalid command text given");
    let mut output = String::from("Executed_");
    output.push_str(&splitted.join("_"));
    output
}

#[cfg(test)]
mod testing {
    use std::collections::HashMap;

    use super::{replace_template_placeholders, CommandToExecute};
    use super::return_dummy_processed_command;
    use crate::core::template::PlaceholderTemplate;

    use super::command_processor::{MockCommandProcessor, CommandOutput};
    type FakeCommandOutput<'l> = PlaceholderTemplate<'l, MockCommandProcessor>;
    fn create_dummmy_command_processor<'a>(
        command_text: &'a str,
        expected_error: Option<String>,
        times: usize,
    ) -> FakeCommandOutput<'a> {
        let mut mock = MockCommandProcessor::new();
        let expected_error = expected_error.clone();
        mock.expect_process()
            .times(times)
            .returning(move |to_process| {
                CommandOutput::new(
                    return_dummy_processed_command(to_process),
                    expected_error.clone(),
                )
            });

        PlaceholderTemplate::Commmand(CommandToExecute::new_with(command_text, mock))
    }

    #[test]
    fn should_complement_template_with_command_processor() {
        let given_templa = r#"
# for {hello}
## Uptime is {how_long}
- Uptime is now {how_long_now} and {echo_error}
## Uptime is {how_long} again
more {hello}
### Not {found}

Should inserted {how_long_error} even with errors
"#;
        let mut map: HashMap<&str, FakeCommandOutput> = HashMap::from_iter(
            vec![
                ("hello", PlaceholderTemplate::DirectValue("world")),
                (
                    "how_long",
                    create_dummmy_command_processor("uptime", None, 1),
                ),
                (
                    "not_to_be_inserted",
                    create_dummmy_command_processor("oh_oh", None, 0),
                ),
                (
                    "how_long_error",
                    create_dummmy_command_processor("uptime xxx", Some("error".to_string()), 1),
                ),
                (
                    "echo_error",
                    create_dummmy_command_processor(
                        "echo",
                        Some("mistake: did something wrong".to_string()),
                        1,
                    ),
                ),
                (
                    "how_long_now",
                    create_dummmy_command_processor("uptime now", None, 1),
                ),
            ]
            .into_iter(),
        );

        // Act
        let actual = replace_template_placeholders(given_templa, &mut map);
        let output = actual.replacement();

        let mut errors: Vec<(String, String)> = actual
            .errors()
            .iter()
            .map(|key_value| {
                let (key, value) = key_value;
                (key.to_owned(), value.to_owned())
            })
            .collect();
        errors.sort_by(|left, right| left.0.cmp(&right.0));

        insta::assert_display_snapshot!(output);
        insta::assert_yaml_snapshot!(errors);
    }
}
