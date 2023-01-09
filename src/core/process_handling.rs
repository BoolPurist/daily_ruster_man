use crate::prelude::*;
use std::path::Path;
use std::process::Command;

const NVIM: &str = "nvim";
pub fn start_process_with(path: &Path) -> AppResult {
    let path_as_str = path
        .to_str()
        .ok_or_else(|| anyhow!("Could not convert path to a text as argument for editor."))?;

    debug!("Starting program {NVIM} with argument: {path_as_str}");

    Command::new(NVIM)
        .arg(path_as_str)
        .spawn()
        .map_err(AppError::new)
        .with_context(|| format!("Failded to start editor {NVIM} with args {path_as_str}"))?
        .wait()
        .map_err(AppError::new)
        .context("Editor did run correctly")?;

    Ok(())
}
