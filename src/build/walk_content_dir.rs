use std::{path, io, fs};
use tera::Tera;
use super::parse_md::{parse_f_metadata_from_md, parse_index_f_metadata, ContentFileMetadata, IndexFileMetadata};

/// recursively walks through the dir and calls cb on files (also parses frontmatter out of md)
pub fn walk_content_dir(
    dir_path: &path::PathBuf,
    tera: &Tera,
    cb: fn(path::PathBuf, &Tera, &Vec<ContentFileMetadata>, &Vec<IndexFileMetadata>) -> std::io::Result<()>,
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
                walk_content_dir(&f_entry.path(), tera, cb)?;
            } else {
                cb(
                    f_entry.path(),
                    tera,
                    &content_f_metadata_vec,
                    &index_f_metadata_vec,
                )?;
            }
        }
    }
    Ok(())
}