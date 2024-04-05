use std::{env, fs, path};

/// Checks for a config file to verify we are in a project dir
/// # Panics
/// Panics if a project config file is not found in the cwd (we are not in a project dir)
pub fn get_project_cwd() -> path::PathBuf {
  let cwd = env::current_dir().expect("You have no cwd?");
  let config_file_path = cwd.clone().join("larvae.yaml");
  
  // likely should do an assert here
  match fs::read_to_string(config_file_path) {
      Ok(_) => {},
      Err(_) => {
          panic!("Couldn't find your config file, are you in the project's dir?")
      }
  };

  cwd
}