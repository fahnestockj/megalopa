use clap::Parser;
use megalopa::cli::{Cli, Command, self};
use megalopa::cms::build;
use megalopa::web_server;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::New { title, content_name } => cli::write_file::write_file(title, content_name),
        Command::Dev { port } => web_server::start_dev_server(port),
        Command::Build {} => build(),
        Command::Init {} => cli::init::init()
    }
}
