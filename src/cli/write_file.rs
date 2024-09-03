use slug;
use std::fs;
use std::io::Write;

use crate::proj_config_utils::get_project_dir;

/// Creates a new md file
pub fn write_file(title: String, mut content_name: String) -> std::io::Result<()> {
    let content_template: String = format!("---\ntitle: {}\n----\n\n#", title);
    if content_name.len() == 0 {
        content_name = title.clone();
    }
    let index_template: String = format!("---\ncontent_name: {}\n----\n\n#", content_name);

    let cwd = get_project_dir();
    let slug = slug::slugify(title);
    let content_dir = cwd.join("content").join(&content_name);
    if !content_dir.exists() {
        // if the dir doesn't exist we should make an index.md with the barebones
        fs::create_dir(&content_dir)?;
        fs::write(content_dir.join("index.md"), index_template)?;
    }
    let mut file_path = cwd.clone().join(content_name).join(slug);
    file_path.set_extension("md");

    let mut file = fs::File::create_new(&file_path)?;
    file.write_all(content_template.as_bytes())
}
