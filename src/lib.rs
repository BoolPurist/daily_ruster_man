#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate getset;
#[macro_use]
extern crate anyhow;
pub type AppResult<T = ()> = anyhow::Result<T>;
pub type AppError = anyhow::Error;
pub mod cli;
pub mod core;
