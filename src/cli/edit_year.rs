use clap::Parser;

use super::edit_argument::EditCommonArgs;

#[derive(Parser, Getters, CopyGetters)]
pub struct EditByYear {
    #[getset(get_copy = "pub")]
    year: Option<u32>,
    #[getset(get = "pub")]
    #[command(flatten)]
    option: EditCommonArgs,
}
