use clap::Parser;
use megalopa::build::build;
use megalopa::cli::{self, Cli, Command};
use megalopa::web_server;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::New {
            title,
            content_name,
        } => cli::write_file::write_file(title, content_name).unwrap(),
        Command::Dev { port } => web_server::start_dev_server(port),
        Command::Build { empty_out_dir } => build(empty_out_dir),
        Command::Init { project_name } => cli::init::init_project(project_name),
    }
}
