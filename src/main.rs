use clap::Parser;
use megalopa::cli;
use megalopa::cli::{Cli, Command};
use megalopa::cms::build;
use megalopa::web_server;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::New { title } => cli::write_file::write_file(title),
        Command::Dev { port } => web_server::start_dev_server(port),
        Command::Build {} => build(),
        Command::Init {} => cli::init::init()
    }
}
