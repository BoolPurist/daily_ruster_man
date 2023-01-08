use std::path::Path;
use std::process::Command;

const NVIM: &str = "nvim";
pub fn start_process_with(path: &Path) {
    let path_as_str = path
        .to_str()
        .expect("Could not convert path to a text as argument for editor.");

    Command::new(NVIM)
        .arg(path_as_str)
        .spawn()
        .expect("Could not spawn editor as child process")
        .wait()
        .expect("editor failed");
}
