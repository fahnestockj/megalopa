use clap::Parser;
use megalopa::cli::{write_file, Command, Cli};


fn main() {
    let cli = Cli::parse();


    match cli.command {
        Command::New { title } => write_file::write_file(title),
    }
}