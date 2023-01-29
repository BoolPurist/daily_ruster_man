pub mod command_processor;
pub use command_processor::{CommandToExecute, OsCommandProcossor};

#[derive(Debug)]
pub enum PlaceholderTemplate<'a, T> {
    DirectValue(&'a str),
    Commmand(CommandToExecute<'a, T>),
}
