#[macro_export]
macro_rules! build_env_name {
    ($field:ident) => {{
        concat!("RUSTER_JOURNAL", concat!("_", stringify!($field)))
    }};
}
pub(crate) use build_env_name;

pub mod app_args;

pub mod deletion_arguments;
pub mod edit_argument;
pub mod edit_command;
pub mod edit_year;
pub mod list_command;
pub mod month_edit_command;
pub mod month_list_command;
pub mod prompt;
