use std::collections::HashMap;
use super::{CommandProcessor, PlaceholderTemplate};
use regex::Replacer;

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

        let key = inner_match.as_str();

        match self.map.get_mut(key) {
            Some(to_insert) => match to_insert {
                PlaceholderTemplate::DirectValue(direct_insert) => {
                    debug!(
                        "Inserted for placeholder key {} direct value {}",
                        key, direct_insert
                    );
                    dst.push_str(direct_insert);
                }
                PlaceholderTemplate::Commmand(command_to_insert) => {
                    let command_output = command_to_insert.get_std_out();

                    dst.push_str(command_output);

                    debug!(
                        "Inserted for placeholder key {} the output {}",
                        key, command_output,
                    );

                    let command_error = command_to_insert.get_std_err();
                    if let Some(stderr) = command_error {
                        if !self.errors.contains_key(key) {
                            debug!("Found error output for key {}. Output: {}", key, stderr);
                            self.errors.insert(key.to_owned(), stderr.to_owned());
                        }
                    }
                }
            },
            None => dst.push_str(full_match.as_str()),
        }
    }
}
