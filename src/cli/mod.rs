use clap::{Parser, Subcommand};
pub mod write_file;
pub mod init;

/// A CLI for growing and curating a crab larva!
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Write something in your larva
    New {
        title: String,
        // defaults to title if not provided (handled in new fn)
        #[arg(default_value_t = String::from(""))]
        content_name: String,
    },
    /// Host a local web server
    Dev {
        #[arg(default_value_t = 3000)]
        port: u16,
    },
    /// Build your md files => html files
    Build {},
    /// Initialize a new project (in cwd)
    Init {
        project_name: String
    },
}
