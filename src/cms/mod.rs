use clap::builder::OsStr;
use markdown;
use std::fs;
use std::io;
use std::path;
use std::path::PathBuf;

use crate::utils::get_project_dir;
/// Run through md files in content and generate html from them!
pub fn build() {
    let proj_content_dir = get_project_dir().join("content");
    walk_dir(proj_content_dir).unwrap();
}

/// Parsed md -> html and writes to the same path in the /public dir
fn parse_file_to_html_and_write_to_build(file_path: path::PathBuf) {
    assert_eq!(file_path.extension().unwrap(), "md");

    let proj_path = get_project_dir();
    let md_str = fs::read_to_string(file_path.clone()).unwrap();
    let html_contents = markdown::to_html(&md_str);
    // we need the relative path after /content from the file_path
    // I guess we pop the pathbuf until we find content?
    let file_path_clone = file_path.clone();
    let mut prefix = PathBuf::new();
    for component in file_path_clone.components() {
        prefix.push(component);
        if component.eq(&path::Component::Normal(&OsStr::from("content"))) {
            break;
        }
    }
    let file_path = file_path_clone.strip_prefix(prefix).unwrap();
    let mut build_file_path = proj_path
        .join("public")
        .join(file_path);
    build_file_path.set_extension("html");
    fs::create_dir_all(build_file_path.parent().unwrap()).unwrap();
    fs::write(build_file_path, html_contents).unwrap();
}
/// recursively walks through the dir and calls parse_file_to_html_and_write_to_build on files
fn walk_dir(dir_path: path::PathBuf) -> io::Result<()> {
    if dir_path.is_dir() {
        for f_entry in fs::read_dir(dir_path)? {
            let f_entry = f_entry?;
            if f_entry.path().is_dir() {
                walk_dir(f_entry.path())?;
            } else {
                parse_file_to_html_and_write_to_build(f_entry.path())
            }
        }
    }
    Ok(())
}
