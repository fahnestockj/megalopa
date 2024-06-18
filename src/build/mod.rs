use markdown;
use parse_md::parse_f_metadata_from_md;
use parse_md::parse_index_f_metadata;
use parse_md::ContentFileMetadata;
use parse_md::IndexFileMetadata;
use std::fs;
use std::io;
use std::path;
use std::vec;
use tera::Tera;

use crate::utils::get_project_dir;
use crate::utils::read_config;
use path_utils::{get_build_path, get_relative_file_path_for_routing};

mod path_utils;
mod parse_md;

/// Run through md files in content and generate html from them!
pub fn build() {
    let proj_content_dir = get_project_dir().join("content");
    let mut tera = match Tera::new("../templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            panic!("Parsing erro(s): {}", e);
        }
    };
    tera.autoescape_on(vec![]);

    walk_dir(&proj_content_dir, &tera, build_md_file).unwrap();
}

/// recursively walks through the dir and calls cb on files (also parses frontmatter out of md)
fn walk_dir(
    dir_path: &path::PathBuf,
    tera: &Tera,
    cb: fn(path::PathBuf, &Tera, &Vec<ContentFileMetadata>, &Vec<IndexFileMetadata>) -> (),
) -> io::Result<()> {
    if dir_path.is_dir() {
        let mut content_f_metadata_vec: Vec<ContentFileMetadata> = vec![];
        let mut index_f_metadata_vec: Vec<IndexFileMetadata> = vec![];
        for f_entry in fs::read_dir(&dir_path)? {
            let f_entry = f_entry?;
            let f_path = f_entry.path();
            if f_path.is_file()
                && f_path.extension().is_some_and(|ext| ext == "md")
                && f_entry.file_name().ne("index.md")
            {
                content_f_metadata_vec.push(parse_f_metadata_from_md(&f_path));
            }
            if f_path.is_dir() {
                index_f_metadata_vec.push(parse_index_f_metadata(&f_path))
            }
        }
        for f_entry in fs::read_dir(&dir_path)? {
            let f_entry = f_entry?;
            if f_entry.path().is_dir() {
                walk_dir(&f_entry.path(), tera, cb)?;
            } else {
                cb(
                    f_entry.path(),
                    tera,
                    &content_f_metadata_vec,
                    &index_f_metadata_vec,
                );
            }
        }
    }
    Ok(())
}
/// md -> html content -> injected into template
/// also writes to the same path in the /public dir
fn build_md_file(
    file_path: path::PathBuf,
    tera: &Tera,
    content_f_metadata_vec: &Vec<ContentFileMetadata>,
    index_f_metadata_vec: &Vec<IndexFileMetadata>,
) -> () {
    assert_eq!(file_path.extension().unwrap(), "md");

    let proj_path = get_project_dir();
    let proj_config = read_config(&proj_path);

    let mut build_file_path = get_build_path(&file_path, &proj_path);

    let md_str = fs::read_to_string(file_path.clone()).unwrap();

    let options = markdown::Options {
        parse: markdown::ParseOptions {
            constructs: markdown::Constructs {
                frontmatter: true,
                ..markdown::Constructs::default()
            },
            ..markdown::ParseOptions::default()
        },
        ..markdown::Options::default()
    };

    let html_contents = markdown::to_html_with_options(&md_str, &options).unwrap();

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
    fs::create_dir_all(build_file_path.parent().unwrap()).unwrap();
    fs::write(build_file_path, file_contents).unwrap();
}

pub fn move_static_file () {
    

}
