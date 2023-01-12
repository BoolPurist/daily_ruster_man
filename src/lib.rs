#![allow(clippy::uninlined_format_args)]

#[macro_use]
extern crate getset;
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate parse_display;

pub mod cli;
pub mod core;

pub type AppResult<T = ()> = anyhow::Result<T>;
pub type AppError = anyhow::Error;

pub mod prelude {
    pub use super::{AppResult, AppError, anyhow::Context};
}
