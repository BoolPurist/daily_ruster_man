#![allow(clippy::uninlined_format_args)]
#![deny(rustdoc::broken_intra_doc_links)]

#[macro_use]
extern crate getset;
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;

pub mod cli;
pub mod core;
pub type AppResult<T = ()> = anyhow::Result<T>;
pub type AppError = anyhow::Error;

pub mod prelude {
    pub use once_cell::sync::OnceCell as SyncOnceCell;
    pub use once_cell::sync::Lazy as SyncLazy;

    pub use once_cell::unsync::OnceCell;
    pub use once_cell::unsync::Lazy;

    pub use super::{AppResult, AppError, anyhow::Context};

    #[macro_export]
    macro_rules! regex {
        ($re:literal $(,)?) => {{
            static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
            RE.get_or_init(|| regex::Regex::new($re).unwrap())
        }};
    }
}
