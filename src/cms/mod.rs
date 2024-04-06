use markdown;
use std::fs::{self, FileType};
use std::path;

use crate::utils::get_project_cwd;
/// Run through md files in content and generate html from them!
pub fn build() {
    let proj_public_dir = get_project_cwd().join("public");
    // TODO: recursive reading to handle nested dir entries?
    match fs::read_dir(proj_public_dir) {
        Ok(dir_entries) => {
            for r_file in dir_entries {
                if let Ok(file) = r_file {
                    if file.path().extension().unwrap().eq("md") {
                        parse_file_to_html_and_write_to_build(file.path());
                    }
                
                }
            }
        }
        Err(e) => eprintln!("Error reading dir: {}", e),
    }
}

/// Parsed md -> html and writes to the same path in the /public dir
fn parse_file_to_html_and_write_to_build(file_path: path::PathBuf) {
    // TODO: will be busted for nested entries when you add them laterr...
    let proj_path = file_path.parent().expect("Why did you make a project in your root dir? dumby...");
    let md_str = fs::read_to_string(file_path.clone()).unwrap();
    let html_contents = markdown::to_html(&md_str);

    let mut build_file_path = proj_path.join("build").join(proj_path.file_stem().unwrap());
    build_file_path.set_extension("md");
    fs::write(build_file_path, html_contents).unwrap();
}


