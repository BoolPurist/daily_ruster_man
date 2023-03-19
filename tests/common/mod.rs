#![allow(dead_code)]

use std::fs::{File, self};
use std::path::{Path, PathBuf};
use tempfile::TempDir;

pub fn create_sample_data_folder() -> TempDir {
    let to_return =
        TempDir::new().expect("Unexpectd: Failed to create temp folder for integration test.");
    let path = to_return.path();

    create_files(
        path,
        &[
            Path::new("2022_yearly.md"),
            Path::new("2023_yearly.md"),
            Path::new("2001_01_monthly.md"),
            Path::new("2001_02_monthly.md"),
            Path::new("2002_02_monthly.md"),
            Path::new("2002_11_monthly.md"),
            Path::new("2023_03_09_daily.md"),
            Path::new("2023_03_08_daily.md"),
            Path::new("2023_02_02_daily.md"),
            Path::new("1999_02_21_daily.md"),
            Path::new("1999_01_21_daily.md"),
            Path::new("1988_11_22_daily.md"),
            Path::new("1788_08_12_daily.md"),
        ],
    );

    return to_return;

    fn create_files(root: &Path, paths: &[&Path]) {
        for to_create in paths {
            let dest_path = root.join(to_create);
            File::create(dest_path)
                .expect("Failed to create temp journal file for integration test.");
        }
    }
}

#[derive(Default)]
pub struct FileTmpBuilder {
    files: Vec<(PathBuf, Option<String>)>,
}

impl FileTmpBuilder {
    pub fn with_file(&mut self, path: PathBuf, content: Option<String>) -> &mut Self {
        self.files.push((path, content));
        self
    }

    pub fn build(&self) -> TempDir {
        let to_return =
            TempDir::new().expect("Unexpectd: Failed to create temp folder for integration test.");
        let path = to_return.path();

        for (file_name, content) in self.files.iter() {
            create_files(path, &file_name, content.clone());
        }

        return to_return;

        fn create_files(root: &Path, file_name: &Path, maybe_content: Option<String>) {
            let dest_path = root.join(file_name);
            _ = File::create(&dest_path)
                .expect("Failed to create temp journal file for integration test.");
            if let Some(content) = maybe_content {
                fs::write(dest_path, content).expect("Could not write content to file");
            }
        }
    }
}
