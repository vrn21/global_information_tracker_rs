use clap::{Parser, Subcommand};
use std::env;
use std::fs;

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
}
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    let args = Args::parse();

    match args.command {
        Command::Init => {
            fs::create_dir(".git").unwrap();
            fs::create_dir(".git/objects").unwrap();
            fs::create_dir(".git/refs").unwrap();
            fs::write(".git/HEAD", "ref: refs/heads/master\n").unwrap();
            println!("Initialized git directory");
        }
    }
}
