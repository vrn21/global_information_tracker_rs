use anyhow::Ok;
use anyhow::{anyhow, bail, ensure};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod commands;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command[subcommand]]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Init,
    CatFile {
        #[clap(short = 'p')]
        pretty_print: bool,
        object_hash: String,
    },
    HashObject {
        #[clap(short = '2')]
        write: bool,
        file: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args.command {
        Command::Init => {
            commands::init::invoke();
        }
        Command::CatFile {
            pretty_print,
            object_hash,
        } => {
            commands::cat_file::invoke(pretty_print, &object_hash);
        }
        Command::HashObject { write, file } => {
            commands::hash_object::invoke(write, file);
        }
    }

    Ok(())
}
