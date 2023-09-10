use std::path::PathBuf;

const IGNORE: [&str; 2] = ["target", "node_modules"];

pub fn ignore_dir(path: &PathBuf) -> bool {
    let path_string = path.display().to_string();
    for ignore in IGNORE {
        if path_string.ends_with(ignore) {
            return true;
        }
    }
    false
}
