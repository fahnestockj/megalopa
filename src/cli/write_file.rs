use std::env;
use std::fs;
use std::io::Write;
use slug;

const TEMPLATE: &[u8; 2] = b"# ";
/// Creates a new md file
pub fn write_file(title: String) {
    let cwd = env::current_dir().expect("You have no cwd?");
    let config_file_path = cwd.clone().join("larvae.yaml");
    
    // likely should do an assert here
    match fs::read_to_string(config_file_path) {
        Ok(_) => {},
        Err(err) => {
            println!("{}", err.kind()); //TODO: handle better
            panic!("Couldn't find your config file, are you in the project's dir?")
        }
    };
    
    // slug the title
    let slug = slug::slugify(title);

    let file_path = cwd.clone().join(slug);

    let mut file = fs::File::create_new(&file_path).unwrap_or_else(|err| {
        println!("error kind: {}", err.kind());
        panic!("AAA")
    });

    file.write_all(TEMPLATE).unwrap();
}
