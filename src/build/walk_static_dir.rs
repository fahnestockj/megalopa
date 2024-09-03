use std::{fs, fs::DirEntry, io, path::Path};

use crate::{build::path_utils::get_relative_file_path, proj_config_utils::get_project_dir};

/// recursively walks through the dir and calls cb on files
pub fn walk_static_dir(dir: &Path, cb: fn(&DirEntry) -> io::Result<()>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                walk_static_dir(&path, cb)?;
            } else {
                cb(&entry)?;
            }
        }
    }
    Ok(())
}

pub fn copy_static_file(dir_entry: &DirEntry) -> io::Result<()> {
    assert!(!dir_entry.path().is_dir());
    let proj_path = get_project_dir();
    let file_path = dir_entry.path();
    
    let file_path_relative_to_static_dir = get_relative_file_path(&file_path, "static");
    let build_path= proj_path.join("public").join(file_path_relative_to_static_dir);
    fs::copy(&file_path, &build_path)?;
    Ok(())
}
