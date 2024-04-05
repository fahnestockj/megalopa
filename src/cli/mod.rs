use clap::{Parser, Subcommand};
pub mod write_file;

/// A CLI for growing and curating a crab larva!
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    //We want a config file... config.yaml - then we can orient our folder structure from there
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Write something in your larva
    New {
        #[arg(short, long)]
        title: String,
    },
}