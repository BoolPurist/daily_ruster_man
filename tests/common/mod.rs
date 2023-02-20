use std::fs::File;
use std::path::Path;
use tempfile::TempDir;

pub fn create_sample_data_folder() -> TempDir {
    let to_return =
        TempDir::new().expect("Unexpectd: Failed to create temp folder for integration test.");
    let path = to_return.path();

    create_files(
        path,
        &[
            Path::new("2022_yearly.md"),
            Path::new("2001_01_monthly.md"),
            Path::new("1988_11_22_daily.md"),
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
