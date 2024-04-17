use std::path::Path;
use notify::{RecursiveMode, Watcher, Result};
use crate::cms::build;

//TODO: what happens if a thread is reading an html file while build is writting it?

/// Will trigger a build when a file is changed
pub fn setup_file_watcher(content_dir: &Path) -> Result<()> {
    // watch for file changes in content
    let mut watcher = notify::recommended_watcher(|res| match res {
        Ok(event) => {
            println!("change detected: {:?}", event);
            build();
        }
        Err(e) => println!("watch error: {:?}", e),
    })?;
    
    watcher
        .watch(content_dir, RecursiveMode::Recursive)
}
