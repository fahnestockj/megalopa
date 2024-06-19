use build_md_file::build_md_file;
use std::vec;
use tera::Tera;
use walk_content_dir::walk_content_dir;
use walk_static_dir::{copy_static_file, walk_static_dir};

use crate::utils::get_project_dir;

mod build_md_file;
mod parse_md;
mod path_utils;
mod walk_content_dir;
mod walk_static_dir;

/// Run through md files in content and generate html from them!
pub fn build() {
    let proj_dir = get_project_dir();
    let mut tera = match Tera::new("../templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            panic!("Parsing erro(s): {}", e);
        }
    };
    tera.autoescape_on(vec![]);

    walk_content_dir(&proj_dir.join("content"), &tera, build_md_file).unwrap();
    walk_static_dir(&std::path::Path::new("../static"), copy_static_file).unwrap();
}
