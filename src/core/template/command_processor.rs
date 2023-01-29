use std::process::Command;

use mockall::*;
use derive_new::new;

use crate::core::template;

#[automock]
pub trait CommandProcessor {
    /// Executes a processs according to a given command with args as text
    /// and returns the output
    fn process(&self, command_text: &str) -> CommandOutput;
}

#[derive(Debug, new, Getters)]
#[getset(get = "pub")]
pub struct CommandOutput {
    /// Output of program as if written to the terminal
    stdout: String,
    /// If there, the program was executed with an error. It is gurantted that the inner string is not
    /// empty and not only whitespaces
    stderr: Option<String>,
}

#[derive(Debug)]
pub struct CommandToExecute<'a, T> {
    provided_command: &'a str,
    output_of_executed: Option<CommandOutput>,
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
            output_of_executed: None,
        }
    }

    pub fn get_std_out(&mut self) -> &str {
        self.ensure_execution_only_once();
        self.output_of_executed
            .as_ref()
            .expect("Unexpected: No command output found even if commmand was executed")
            .stdout()
            .as_str()
    }
    pub fn get_std_err(&mut self) -> Option<&str> {
        self.ensure_execution_only_once();
        let std_err = self
            .output_of_executed
            .as_ref()
            .expect("Unexpected: No command output found even if commmand was executed");
        std_err.stderr().as_deref()
    }

    fn ensure_execution_only_once(&mut self) {
        if self.output_of_executed.is_none() {
            let output = self.command_processor.process(self.provided_command);
            self.output_of_executed = Some(output);
        }
    }
}

#[derive(Default, Debug)]
pub struct OsCommandProcossor;
impl CommandProcessor for OsCommandProcossor {
    fn process(&self, command_text: &str) -> CommandOutput {
        return match template::parse_commmand_text(command_text) {
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
                                let stderr = String::from_utf8_lossy(&err).to_string();
                                CommandOutput::new(
                                    String::from_utf8_lossy(&out).to_string(),
                                    if stderr.trim().is_empty() {
                                        None
                                    } else {
                                        Some(stderr)
                                    },
                                )
                            }
                        }
                    }
                    _ => return_error("No command provided".to_string()),
                }
            }
        };

        fn return_error(error: String) -> CommandOutput {
            println!("asdfas");
            CommandOutput::new(String::new(), Some(error))
        }
    }
}

#[cfg(test)]
pub mod testing {

    use mockall::predicate;
    use crate::core::template;
    use super::{CommandOutput, MockCommandProcessor, CommandToExecute};

    #[test]
    fn should_invoke_only_once_command() {
        let command_text = "echo 'hello world'";

        let expected_error = Some("Some error".to_owned());
        let expected_stdout = "Executed_echo_hello world";

        let mut mock = MockCommandProcessor::new();
        let expected_error_cloned = expected_error.clone();
        mock.expect_process()
            .with(predicate::eq(command_text))
            .times(1)
            .returning(move |to_process| {
                CommandOutput::new(
                    template::return_dummy_processed_command(to_process),
                    expected_error_cloned.clone(),
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
        assert_eq!(
            expected_error.expect("Unexpected: actual stderr should not be none"),
            actual_strerr.expect("Unexpected: expected stderr should not be none")
        );
    }
}
