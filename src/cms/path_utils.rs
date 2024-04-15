use std::path::{self, PathBuf};
use std::ffi::OsStr;

/// get an absolute path to public/<relative path from /content>
pub fn get_build_path(file_path: &PathBuf, proj_path: &PathBuf) -> PathBuf {
    let relative_path = get_relative_file_path(&file_path, "content");
    let mut absolute_path = PathBuf::from(proj_path);
    absolute_path.push("public");
    absolute_path.push(relative_path);
    absolute_path
}

/// given an absolute file path get relative path after provided dir name
pub fn get_relative_file_path(content_path_ref: &PathBuf, from: &str) -> PathBuf {
    let content_path = content_path_ref.clone();
    let mut prefix = PathBuf::new();
    for component in content_path.components() {
        prefix.push(component);
        if component.eq(&path::Component::Normal(&OsStr::new(from))) {
            break;
        }
    }
    assert_ne!(prefix, content_path);

    content_path.strip_prefix(prefix).unwrap().to_path_buf()
}

/// given an absolute path get relative path after provided dir name with no file extension
pub fn get_relative_file_path_for_routing(content_path_ref: &PathBuf, from: &str) -> PathBuf {
    let mut relative_path = get_relative_file_path(content_path_ref, from);
    relative_path.set_extension("");
    relative_path
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn relative_path() {
        // TODO

    }
}
