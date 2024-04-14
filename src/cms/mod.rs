use clap::builder::OsStr;
use markdown;
use std::fs;
use std::io;
use std::path;
use std::path::PathBuf;
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
    // We're relying on the markdown package to safely escape user markdown content
    // (which is all we support for now)
    tera.autoescape_on(vec![]);

    walk_dir(proj_content_dir, &tera, build_file).unwrap();
}

/// recursively walks through the dir and calls cb on files
fn walk_dir(
    dir_path: path::PathBuf,
    tera: &Tera,
    cb: fn(path::PathBuf, &Tera) -> (),
) -> io::Result<()> {
    if dir_path.is_dir() {
        for f_entry in fs::read_dir(dir_path)? {
            let f_entry = f_entry?;
            if f_entry.path().is_dir() {
                walk_dir(f_entry.path(), tera, cb)?;
            } else {
                cb(f_entry.path(), tera);
            }
        }
    }
    Ok(())
}

/// md -> html content -> injected into template
/// also writes to the same path in the /public dir
fn build_file(file_path: path::PathBuf, tera: &Tera) -> () {
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
            file_contents = tera.render("homepage.html", &context).unwrap();
        } else {
            // build index page
            file_contents = tera.render("index.html", &context).unwrap();
        }
    } else {
        // build content page
        context.insert("content_title", "HeeHoo"); //TODO:
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
    proj_path.join("public").join(file_path)
}
