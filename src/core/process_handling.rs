use crate::{prelude::*, core::constants::NVIM};

use std::path::Path;
use std::process::Command;

use super::app_options::AppOptions;

pub fn start_process_with(option: &AppOptions, path: &Path) -> AppResult {
    let path_as_str = path
        .to_str()
        .ok_or_else(|| anyhow!("Could not convert path to a text as argument for editor."))?;

    debug!("Starting program {} with argument: {}", NVIM, path_as_str);

    if !option.run_editor_dry() {
        Command::new(NVIM)
            .arg(path_as_str)
            .spawn()
            .map_err(AppError::new)
            .with_context(|| format!("Failded to start editor {NVIM} with args {path_as_str}"))?
            .wait()
            .map_err(AppError::new)
            .context("Editor did run correctly")?;
    }

    Ok(())
}
