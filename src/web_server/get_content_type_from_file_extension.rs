use std::{collections::HashMap, ffi::OsStr};
pub fn get_content_type_from_file_extension(extension: &OsStr) -> &str {
    let supported_content_types: HashMap<&str, &str> = HashMap::from([
        ("js", "text/javascript"),
        ("html", "text/html"),
        ("css", "text/css"),
        ("svg", "image/svg+xml")
    ]);
    let ext_key = extension
        .to_str()
        .expect("extension couldn't be turned a string");

    let content_type = supported_content_types.get(ext_key);
    match content_type {
        Some(header) => header,
        None => panic!("File extension {} not supported", ext_key),
    }
}
