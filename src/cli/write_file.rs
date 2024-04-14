use std::fs;
use std::io::Write;
use slug;

use crate::utils::get_project_dir;

const TEMPLATE: &[u8; 2] = b"# ";
/// Creates a new md file
pub fn write_file(title: String) {

    let cwd = get_project_dir();
    
    // slug the title
    let slug = slug::slugify(title);

    let file_path = cwd.clone().join(slug);

    let mut file = fs::File::create_new(&file_path).unwrap_or_else(|err| {
        println!("error kind: {}", err.kind());
        panic!("AAA")
    });

    file.write_all(TEMPLATE).unwrap();
}
