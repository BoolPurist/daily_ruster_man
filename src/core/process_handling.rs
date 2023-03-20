use crate::prelude::*;

use std::cell::RefCell;
use std::path::Path;
use std::process::Command;

use super::app_options::AppOptions;

pub trait ProcessExecuter {
    fn start_program(&self, option: &AppOptions, editor: &str, path: &Path) -> AppResult;
}

#[derive(Default)]
pub struct RealProcessExecuter {}

impl ProcessExecuter for RealProcessExecuter {
    fn start_program(&self, option: &AppOptions, editor: &str, path: &Path) -> AppResult {
        let path_as_str = path
            .to_str()
            .ok_or_else(|| anyhow!("Could not convert path to a text as argument for editor."))?;

        debug!("Starting program {} with argument: {}", editor, path_as_str);

        if !option.run_editor_dry() {
            Command::new(editor)
                .arg(path_as_str)
                .spawn()
                .map_err(AppError::new)
                .with_context(|| {
                    format!(
                        "Failded to start editor {0} with args {path_as_str}
Does {0} as an editor exits and is findable via $PATH ?",
                        editor
                    )
                })?
                .wait()
                .map_err(AppError::new)
                .context("Editor did run correctly")?;
        }

        Ok(())
    }
}

#[derive(Default)]
pub struct TestProcessExecuter {
    last_executed_program: RefCell<String>,
}

impl TestProcessExecuter {
    pub fn get_last_executed_program(&self) -> String {
        self.last_executed_program.borrow().to_owned()
    }
}

impl ProcessExecuter for TestProcessExecuter {
    fn start_program(&self, _option: &AppOptions, editor: &str, path: &Path) -> AppResult {
        let mut mut_executed_program = self.last_executed_program.borrow_mut();
        *mut_executed_program = format!("{} {}", editor, path.to_string_lossy());
        Ok(())
    }
}
