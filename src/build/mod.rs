use std::vec;
use tera::Tera;
use walk_content_dir::walk_content_dir;
use build_md_file::build_md_file;

use crate::utils::get_project_dir;


mod path_utils;
mod parse_md;
mod build_md_file;
mod walk_content_dir;

/// Run through md files in content and generate html from them!
pub fn build() {
    let proj_content_dir = get_project_dir().join("content");
    let mut tera = match Tera::new("../theme/templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            panic!("Parsing erro(s): {}", e);
        }
    };
    tera.autoescape_on(vec![]);

    walk_content_dir(&proj_content_dir, &tera, build_md_file).unwrap();
}


pub fn move_static_file () {
    

}
