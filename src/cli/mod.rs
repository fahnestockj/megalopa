use color_eyre::eyre::{eyre, Result, WrapErr};
use directories::UserDirs;
use std::path::PathBuf;
use structopt::StructOpt;

mod write;

/// A CLI for growing and curating a crab larva!
#[derive(StructOpt, Debug)]
#[structopt(name = "megalopa")]
struct Opt {
    //parse the path string
    #[structopt(parse(from_os_str), short = "p", long, env)]
    crustacean_path: Option<PathBuf>,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// Write something in your larva
    Write {},
}

fn get_default_crustacean_dir() -> Result<PathBuf> {
    let user_dirs = UserDirs::new().ok_or_else(|| {
        eyre!("Could not get home directory. Are you sure you are on a computer?")
    })?;
    //TODO: What if the dir doesn't exist??!!
    // we'd likely want to create a temp directory then yah?
    // or we could create a permanent one... and not worry about it...
    Ok(user_dirs.home_dir().join(".crustacean"))
}
