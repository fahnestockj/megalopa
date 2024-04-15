use clap::builder::OsStr;
use markdown;
use markdown::mdast::Node;
use serde::Deserialize;
use serde::Serialize;
use serde_yaml;
use std::fs;
use std::io;
use std::path;
use std::path::PathBuf;
use std::vec;
use tera::Tera;

use crate::utils::get_project_dir;
use crate::utils::read_config;
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

    walk_dir(&proj_content_dir, &tera, build_file).unwrap();
}

/// recursively walks through the dir and calls cb on files
fn walk_dir(
    dir_path: &path::PathBuf,
    tera: &Tera,
    cb: fn(path::PathBuf, &Tera, &Vec<FileMetadata>, &Vec<DirMetadata>) -> (),
) -> io::Result<()> {
    if dir_path.is_dir() {
        let mut f_metadata_vec: Vec<FileMetadata> = vec![];
        let mut dir_metadata_vec: Vec<DirMetadata> = vec![];
        for f_entry in fs::read_dir(&dir_path)? {
            let f_entry = f_entry?;
            let f_path = f_entry.path();
            if f_path.is_file() && f_path.extension().is_some_and(|ext| ext == "md") {
                f_metadata_vec.push(parse_f_metadata_from_md(&f_path));
            }
            if f_path.is_dir() {
                dir_metadata_vec.push(parse_metadata_from_dir(&f_path))
            }
        }
        for f_entry in fs::read_dir(&dir_path)? {
            let f_entry = f_entry?;
            if f_entry.path().is_dir() {
                walk_dir(&f_entry.path(), tera, cb)?;
            } else {
                cb(f_entry.path(), tera, &f_metadata_vec, &dir_metadata_vec);
            }
        }
    }
    Ok(())
}

/// md -> html content -> injected into template
/// also writes to the same path in the /public dir
fn build_file(
    file_path: path::PathBuf,
    tera: &Tera,
    f_metadata_vec: &Vec<FileMetadata>,
    dir_metadata_vec: &Vec<DirMetadata>,
) -> () {
    assert_eq!(file_path.extension().unwrap(), "md");

    let proj_path = get_project_dir();
    let proj_config = read_config(&proj_path);

    let build_file_path = get_relative_path_from_content(&file_path, &proj_path);

    let md_str = fs::read_to_string(file_path.clone()).unwrap();
    let html_contents = markdown::to_html(&md_str);

    let mut context = tera::Context::new();
    context.insert("content", &html_contents);
    context.insert("title", &proj_config.title);

    let file_contents: String;

    // we need to decide what type of file this is... then use the corresponding template
    if build_file_path.ends_with("index.html") {
        // root index.html is the homepage - so it will be "public/index.html"
        if let None = build_file_path.parent().unwrap().parent() {
            // build homepage
            context.insert("dir_metadata_vec", dir_metadata_vec);
            file_contents = tera.render("homepage.html", &context).unwrap();
        } else {
            // build index page
            context.insert("file_metadata_vec", f_metadata_vec);
            file_contents = tera.render("index.html", &context).unwrap();
        }
    } else {
        // build content page
        let f_metadata = f_metadata_vec.into_iter().find(|f_metadata| f_metadata.path.eq(&file_path)).unwrap();
        context.insert("content_title", &f_metadata.title);
        file_contents = tera.render("content.html", &context).unwrap();
    }
    fs::create_dir_all(build_file_path.parent().unwrap()).unwrap();
    fs::write(build_file_path, file_contents).unwrap();
}

/// Find the relative path using /content as the root
fn get_relative_path_from_content(file_path: &PathBuf, proj_path: &PathBuf) -> PathBuf {
    let file_path_clone = file_path.clone();
    let mut prefix = PathBuf::new();
    for component in file_path_clone.components() {
        prefix.push(component);
        if component.eq(&path::Component::Normal(&OsStr::from("content"))) {
            break;
        }
    }
    let file_path = file_path_clone.strip_prefix(prefix).unwrap();
    let mut relative_path = proj_path.join("public").join(file_path);
    relative_path.set_extension(".html");
    relative_path
}

#[derive(Serialize)]
struct FileMetadata {
    title: String,
    path: PathBuf,
}
#[derive(Deserialize)]
struct MdContentFileFrontmatter {
    title: Option<String>,
}
/// Parses frontmatter as Yaml for the given generic schema
fn parse_frontmatter_from_md<T: for<'a> serde::Deserialize<'a>>(
    f_str: &String,
) -> Result<T, serde_yaml::Error> {
    let options = markdown::ParseOptions {
        constructs: markdown::Constructs {
            frontmatter: true,
            ..markdown::Constructs::default()
        },
        ..markdown::ParseOptions::default()
    };
    let root_node = &markdown::to_mdast(&f_str, &options).unwrap();
    let yaml_str = &root_node.children().unwrap()[0].to_string();
    serde_yaml::from_str::<T>(&yaml_str)
}

/// defaults to the file name as a title if one isn't found
fn parse_f_metadata_from_md(f_path: &PathBuf) -> FileMetadata {
    let md_str = fs::read_to_string(&f_path).unwrap();
    // try to parse from frontmatter
    if let Ok(frontmatter) = parse_frontmatter_from_md::<MdContentFileFrontmatter>(&md_str) {
        if let Some(file_title) = frontmatter.title {
            return FileMetadata {
                title: file_title,
                path: f_path.clone(),
            };
        }
    }
    // default to the filename
    return FileMetadata {
        title: String::from(f_path.file_stem().unwrap().to_str().unwrap()),
        path: f_path.clone(),
    };
}

#[derive(Serialize)]
struct DirMetadata {
    content_name: String,
    path: PathBuf,
}
#[derive(Deserialize)]
struct MdIndexFileFrontmatter {
    content_title: Option<String>,
}
/// parses content name from dir - is this in index.md frontmatter? if it is it needs a different variable name...
fn parse_metadata_from_dir(dir_path: &path::PathBuf) -> DirMetadata {
    // we currently store dir metadata on the index.md under the var "content_name" in the frontmatter
    let index_f_path = dir_path.join("index.md");
    let md_str = fs::read_to_string(index_f_path).unwrap();
    if let Ok(frontmatter) = parse_frontmatter_from_md::<MdIndexFileFrontmatter>(&md_str) {
        if let Some(content_name) = frontmatter.content_title {
            return DirMetadata {
                content_name,
                path: dir_path.clone(),
            };
        };
    };
    return DirMetadata {
        content_name: String::from(dir_path.file_stem().unwrap().to_str().unwrap()),
        path: dir_path.clone(),
    };
}

#[cfg(test)]
mod tests {
    use markdown::{Constructs, ParseOptions};

    use super::*;

    #[test]
    fn md_parse() {
        let file_str = "---\ntitle: Hello World\ntest: true\n---\n\n# heading 1\n## heading 2\n\ncontent content woo\n";
        let options = ParseOptions {
            constructs: Constructs {
                frontmatter: true,
                ..Constructs::default()
            },
            ..ParseOptions::default()
        };
        let ast = markdown::to_mdast(&file_str, &options).unwrap();
        let first_node = &ast.children().unwrap()[0];
        assert_eq!(first_node.to_string(), "title: Hello World\ntest: true")
    }
}
