use std::{path, fs, io};
use tera::Tera;
use crate::{build::path_utils::{get_relative_file_path, get_relative_file_path_for_routing}, markdown_parsing::parse_markdown, proj_config_utils::{get_project_dir, read_config}};
use super::parse_md::{ContentFileMetadata, IndexFileMetadata};

/// md -> html content -> injected into template
/// also writes to the same path in the /public dir
pub fn build_md_file(
    file_path: path::PathBuf,
    tera: &Tera,
    content_f_metadata_vec: &Vec<ContentFileMetadata>,
    index_f_metadata_vec: &Vec<IndexFileMetadata>,
) -> io::Result<()> {
    assert_eq!(file_path.extension().unwrap(), "md");

    let proj_path = get_project_dir();
    let proj_config = read_config(&proj_path);

    let file_path_relative_to_content_dir = get_relative_file_path(&file_path, "content");
    let mut build_file_path = proj_path.join("public").join(file_path_relative_to_content_dir);

    let md_str = fs::read_to_string(file_path.clone())?;

    let html_contents = parse_markdown(&md_str);

    let mut context = tera::Context::new();
    context.insert("content", &html_contents);
    context.insert("title", &proj_config.title);

    let file_contents: String;

    // we need to decide what type of file this is... then use the corresponding template
    // easier to reason with the relative path
    let relative_path = get_relative_file_path_for_routing(&build_file_path, "public");
    if relative_path.ends_with("index") {
        if let None = relative_path.parent().unwrap().parent() {
            // build homepage
            context.insert("dir_metadata_vec", index_f_metadata_vec);
            file_contents = tera.render("homepage.html", &context).unwrap();
        } else {
            // build index page
            context.insert("file_metadata_vec", content_f_metadata_vec);
            file_contents = tera.render("index.html", &context).unwrap();
        }
    } else {
        // build content page
        let f_metadata = content_f_metadata_vec
            .into_iter()
            .find(|f_metadata| f_metadata.path.eq(&relative_path))
            .unwrap();
        context.insert("content_title", &f_metadata.title);
        file_contents = tera.render("content.html", &context).unwrap();
    }

    build_file_path.set_extension("html");
    fs::create_dir_all(build_file_path.parent().unwrap())?;
    fs::write(build_file_path, file_contents)?;
    Ok(())
}