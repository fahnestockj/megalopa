use std::env;
use std::fs;
use std::path;
/// Initialize a project in the current dir
/// # Panics
/// Panics if a file config file exists in the current dir
pub fn init_project(project_name: String) {
    let cwd = env::current_dir().expect("You have no cwd?");
    let config_file_path = cwd.clone().join("larvae.yaml");
    fs::read(config_file_path).expect_err("You already have a project in this dir");

    // write a whole bunch of files...
    create_config_file(&cwd, project_name);
    create_gitignore(&cwd);
    create_dirs_and_homepage(&cwd);
}

fn create_config_file(cwd: &path::PathBuf, project_name: String) {
    let path = cwd.clone().join("larvae.yaml");
    let contents = format!("title: {project_name}\n");
    fs::write(path, contents).expect("Failure creating config file");
}

fn create_gitignore(cwd: &path::PathBuf) {
    let path = cwd.clone().join(".gitignore");
    let contents = format!("/public");
    fs::write(path, contents).expect("Failure creating gitignore");
}

fn create_dirs_and_homepage(cwd: &path::PathBuf) {
    let path = cwd.clone();
    fs::create_dir(path.join("content")).expect("Failure creating content dir");

    let homepage_contents = format!("# Hello World\n");
    fs::write(path.join("content/index.md"), homepage_contents)
        .expect("Failure creating hello-world.md");
    fs::create_dir(path.join("content/example-content")).expect("Failure creating example content dir");

    let example_index_page = format!("---\ncontent_name: example-content\n---\n # Hi from a content index file");
    fs::write(path.join("content/example-content/index.md"), example_index_page)
        .expect("Failure creating example-content/index.md");

    fs::create_dir(path.join("public")).expect("Failure creating public dir");
}
