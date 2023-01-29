use mockall::*;
use crate::prelude::*;

use std::process::Command;
fn parse_commmand_text(to_parse: &str) -> AppResult<Vec<String>> {
    shellwords::split(to_parse).map_err(AppError::new)
}
#[derive(Debug)]
pub struct CommandToExecute<'a, T> {
    provided_command: &'a str,
    stdout_command: Option<String>,
    /// If none the command was not yet executed or an error happened executing the command.
    /// An error counts for either a not valid command text or the execution of the command
    /// produced output in stderr
    error_output: Option<String>,
    command_processor: T,
}

impl<'a> CommandToExecute<'a, OsCommandProcossor> {
    pub fn new(provided_command: &'a str) -> Self {
        Self::init(provided_command, OsCommandProcossor::default())
    }
}

impl<'a, T> CommandToExecute<'a, T>
where
    T: CommandProcessor,
{
    pub fn new_with(provided_command: &'a str, command_processor: T) -> Self {
        Self::init(provided_command, command_processor)
    }

    fn init(provided_command: &'a str, command_processor: T) -> Self {
        Self {
            provided_command,
            command_processor,
            stdout_command: None,
            error_output: None,
        }
    }

    pub fn get_std_out(&mut self) -> &str {
        self.ensure_execution_only_once();
        self.stdout_command.as_ref().unwrap()
    }
    pub fn get_std_err(&mut self) -> &str {
        self.ensure_execution_only_once();
        self.error_output.as_ref().unwrap().trim()
    }

    fn ensure_execution_only_once(&mut self) {
        if self.stdout_command.is_none() {
            let (out, err) = self.command_processor.process(self.provided_command);
            self.stdout_command = Some(out);
            self.error_output = Some(err);
        }
    }
}
#[automock]
pub trait CommandProcessor {
    fn process(&self, command_text: &str) -> (String, String);
}

#[derive(Default, Debug)]
pub struct OsCommandProcossor;
impl CommandProcessor for OsCommandProcossor {
    fn process(&self, command_text: &str) -> (String, String) {
        return match parse_commmand_text(command_text) {
            Err(error) => return_error(error.to_string()),
            Ok(command_args) => {
                let mut iter_command_args = command_args.iter();
                let first = iter_command_args.next();
                match (first, iter_command_args) {
                    (Some(command), rest) => {
                        let mut program = Command::new(command);
                        program.args(rest);

                        match program.output() {
                            Err(error) => return_error(error.to_string()),
                            Ok(out_err) => {
                                let (out, err) = (out_err.stdout, out_err.stderr);
                                (
                                    String::from_utf8_lossy(&out).to_string(),
                                    String::from_utf8_lossy(&err).to_string(),
                                )
                            }
                        }
                    }
                    _ => return_error("No command provided".to_string()),
                }
            }
        };

        fn return_error(error: String) -> (String, String) {
            println!("asdfas");
            (String::new(), error)
        }
    }
}
#[cfg(test)]
pub mod testing {

    use super::*;
    use mockall::predicate::*;
    pub fn return_dummy_processed_command(input: &str) -> String {
        let splitted = super::parse_commmand_text(input).expect("Invalid command text given");
        let mut output = String::from("Executed_");
        output.push_str(&splitted.join("_"));
        output
    }

    #[test]
    fn should_invoke_only_once_command() {
        let command_text = "echo 'hello world'";

        let expected_error = "Some error";
        let expected_stdout = "Executed_echo_hello world";

        let mut mock = MockCommandProcessor::new();
        mock.expect_process()
            .with(predicate::eq(command_text))
            .times(1)
            .returning(|to_process| {
                (
                    return_dummy_processed_command(to_process),
                    expected_error.to_owned(),
                )
            });
        // Act
        let mut actual = CommandToExecute::new_with(command_text, mock);

        // Testing if command is executed only once
        let _ = actual.get_std_out();

        // Assertion
        let actual_stdout = actual.get_std_out();
        assert_eq!(expected_stdout, actual_stdout);

        let actual_strerr = actual.get_std_err();
        assert_eq!(expected_error, actual_strerr);
    }
}
