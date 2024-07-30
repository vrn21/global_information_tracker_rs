use anyhow::{anyhow, bail, ensure};
use anyhow::{Context, Ok, Result};
use clap::{Parser, Subcommand};
use flate2::read::ZlibDecoder;
use std::ffi::CStr;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

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
}

enum Kind {
    Blob,
}

fn main() -> anyhow::Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    let args = Args::parse();

    match args.command {
        Command::Init => {
            fs::create_dir(".git")?;
            fs::create_dir(".git/objects")?;
            fs::create_dir(".git/refs")?;
            fs::write(".git/HEAD", "ref: refs/heads/master\n")?;
            println!("Initialized git directory");
        }
        Command::CatFile {
            pretty_print,
            object_hash,
        } => {
            ensure!(pretty_print, "pretty print must be given (-p)");

            //check length
            let f = std::fs::File::open(format!(
                ".git/objects/{}/{}",
                &object_hash[..2],
                &object_hash[2..]
            ))
            .context("open in .git/")?;

            let mut z = ZlibDecoder::new(f);
            let mut s = String::new();
            let mut buf: Vec<u8> = Vec::new();
            let mut z = BufReader::new(z);
            z.read_until(0, &mut buf)
                .context("reading from ./git/objects/")?;
            let header = CStr::from_bytes_with_nul(&buf);
            let header = header?.to_str().context("it isnt valid utf-8")?;
            let Some((kind, size)) = header.split_once(' ') else {
                anyhow::bail!("the contents didnt have the 'blob' header, contents: \n {header}")
            };

            let kind = match kind {
                "blob" => Kind::Blob,
                _ => bail!("we do not yet know how to print a {kind}"),
            };

            let size = size.parse::<usize>().context("not a valid blob size ")?;
            buf.clear();
            buf.resize(size, 0);
            z.read_exact(&mut buf[..])
                .context("read true contents of .git/objects file")?;
            let n = z
                .read(&mut buf)
                .context("validate eof in ./git/objects file")?;
            anyhow::ensure!(n == 0, "git object had {n} trailing bytes");
            let mut std = std::io::stdout();
            std.lock();

            match kind {
                Kind::Blob => std.write_all(&buf).context("writing objects to stdout")?,
            }
        }
    }
    Ok(())
}
