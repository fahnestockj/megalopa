use std::fs;
use std::io::Write;
use slug;

use crate::utils::get_project_dir;

/// Creates a new md file
pub fn write_file(title: String, content_name: String) {
    
    let template: String = format!("---\ntitle: {}\n----\n\n#", title);

    let cwd = get_project_dir();
    
    // slug the title
    let slug = slug::slugify(title);

    let mut file_path = cwd.clone().join(content_name).join(slug);
    file_path.set_extension("md");

    let mut file = fs::File::create_new(&file_path).unwrap();

    file.write_all(template.as_bytes()).unwrap();
}
