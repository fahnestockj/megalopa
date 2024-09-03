use serde_yaml;
use serde::Deserialize;
use std::{env, fs, path};

/// Checks for a config file to verify we are in a project dir
/// # Panics
/// Panics if a project config file is not found in the cwd (we are not in a project dir)
pub fn get_project_dir() -> path::PathBuf {
    let cwd = env::current_dir().expect("You have no cwd?");
    let config_file_path = cwd.join("larvae.yaml");

    // likely should do an assert here
    match fs::read_to_string(config_file_path) {
        Ok(_) => {}
        Err(_) => {
            panic!("Couldn't find your config file, are you in the project's root dir?")
        }
    };

    cwd
}
#[derive(Debug, Deserialize)]
pub struct Config {
    pub title: String,
}
/// Reads larvae.yaml
/// # Panics
/// Panics if a project config file is not found in the cwd (we are not in a project dir)
pub fn read_config(project_dir: &path::PathBuf) -> Config {
    let config_file_path = project_dir.join("larvae.yaml");
    if let Ok(yaml_str) = fs::read_to_string(config_file_path) {
        let config: Config = serde_yaml::from_str(&yaml_str).unwrap();
        config
    } else {
        panic!("Couldn't find your config file, are you in the project's root dir?")
    }
}
