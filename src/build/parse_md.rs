use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs;
use std::path::{self, PathBuf};

use crate::markdown_parsing::parse_frontmatter;

use super::path_utils::get_relative_file_path_for_routing;

#[derive(Debug, Serialize)]
pub struct ContentFileMetadata {
    pub title: String,
    pub path: PathBuf,
}
#[derive(Debug, Deserialize)]
pub struct MdContentFileFrontmatter {
    pub title: Option<String>,
}
/// Parses frontmatter as Yaml for the given generic schema
pub fn parse_frontmatter_from_md<T: for<'a> serde::Deserialize<'a>>(
    md_str: &str,
) -> Result<T, serde_yaml::Error> {
    let o_frontmatter = parse_frontmatter(md_str);
    if let Some(frontmatter) = o_frontmatter {
        serde_yaml::from_str::<T>(&frontmatter)
    } else {
        serde_yaml::from_str("")
    }
}

/// defaults to the file name as a title if one isn't found
pub fn parse_f_metadata_from_md(f_path: &PathBuf) -> ContentFileMetadata {
    let md_str = fs::read_to_string(&f_path).unwrap();
    // try to parse from frontmatter
    if let Ok(frontmatter) = parse_frontmatter_from_md::<MdContentFileFrontmatter>(&md_str) {
        if let Some(file_title) = frontmatter.title {
            return ContentFileMetadata {
                title: file_title,
                path: get_relative_file_path_for_routing(f_path, "content"),
            };
        }
    }
    // default to the filename
    return ContentFileMetadata {
        title: String::from(f_path.file_stem().unwrap().to_str().unwrap()),
        path: get_relative_file_path_for_routing(f_path, "content"),
    };
}

#[derive(Serialize)]
pub struct IndexFileMetadata {
    pub content_name: String,
    pub path: PathBuf,
}
#[derive(Deserialize)]
pub struct MdIndexFileFrontmatter {
    pub content_title: Option<String>,
}
/// parses content name from dir - is this in index.md frontmatter? if it is it needs a different variable name...
pub fn parse_index_f_metadata(dir_path: &path::PathBuf) -> IndexFileMetadata {
    // we currently store dir metadata on the index.md under the var "content_name" in the frontmatter
    let index_f_path = dir_path.join("index.md");
    let md_str = fs::read_to_string(index_f_path).expect("Couldn't find an index.md in dir");
    if let Ok(frontmatter) = parse_frontmatter_from_md::<MdIndexFileFrontmatter>(&md_str) {
        if let Some(content_name) = frontmatter.content_title {
            return IndexFileMetadata {
                content_name,
                path: get_relative_file_path_for_routing(&dir_path, "content"),
            };
        };
    };
    return IndexFileMetadata {
        content_name: String::from(dir_path.file_stem().unwrap().to_str().unwrap()),
        path: get_relative_file_path_for_routing(&dir_path, "content"),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn md_parse() {
        let file_str = "---\ntitle: Hello World\ntest: true\n---\n\n# heading 1\n## heading 2\n\ncontent content woo\n";
        let frontmatter = parse_frontmatter_from_md::<MdContentFileFrontmatter>(&file_str).unwrap();
        assert_eq!(frontmatter.title.unwrap(), "Hello World")
    }
}
