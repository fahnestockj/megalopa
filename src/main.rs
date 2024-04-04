use color_eyre::eyre::{eyre, Result, WrapErr};
use megalopa::cli::write;
use directories::UserDirs;
use std::path::PathBuf;
use structopt::StructOpt;


fn main() -> Result<> {
    color_eyre::install()?;
    let opt = Opt::from_args();
    let garden_path = match opt.garden_path {
        //if the user passed in a path, use that
        Some(pathbuf) => Ok(pathbuf),
        //otherwise, use the default
        None => get_default_garden_dir().wrap_err("garden_path was not supplied"),
    }?;

    match opt.cmd {
        Command::Write { title } => write(garden_path, title),
    }
}