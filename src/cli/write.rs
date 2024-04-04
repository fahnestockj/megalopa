use std::fs::File;
use std::path::{PathBuf, Path};

const TEMPLATE: &[u8; 2] = b"# ";
/// Creates a new md file
/// # Panics
/// Panics if a file with the same name already exists or if the specified path does not exist
pub fn write(crustacean_path: PathBuf, title_slug: &str) -> Result<String, &'static str> {
    println!("{:?}", crustacean_path.to_str());
    let file_path = String::from(crustacean_path.to_str().unwrap());
    file_path.push_str(title_slug);
    file_path.push_str(".md");
    let final = Path::from(file_path);


    let file = File::create_new(file_path.push_str(&file_name)).unwrap();
}
