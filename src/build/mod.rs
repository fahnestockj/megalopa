use build_md_file::build_md_file;
use std::{
    fs::{self, remove_dir_all},
    vec,
};
use tera::Tera;
use walk_content_dir::walk_content_dir;
use walk_static_dir::{copy_static_file, walk_static_dir};

use crate::proj_config_utils::get_project_dir;

mod build_md_file;
mod parse_md;
mod path_utils;
mod walk_content_dir;
mod walk_static_dir;

/// Run through md files in content and generate html from them!
pub fn build(empty_out_dir: bool) {
    let proj_dir = get_project_dir();
    let mut tera = match Tera::new("../templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            panic!("Parsing erro(s): {}", e);
        }
    };
    tera.autoescape_on(vec![]);
    // clear out stale files
    if empty_out_dir {
        remove_dir_all("public").unwrap();
        fs::create_dir("public").unwrap();
    }
    walk_content_dir(&proj_dir.join("content"), &tera, build_md_file).unwrap();
    walk_static_dir(&std::path::Path::new("../static"), copy_static_file).unwrap();
}
