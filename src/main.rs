use anyhow::{anyhow, bail, ensure};
use anyhow::{Context, Ok, Result};
use clap::{Parser, Subcommand};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};
use std::ffi::CStr;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::{self, Error};
use std::path::Path;
use std::path::PathBuf;

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

enum Kind {
    Blob,
    //Commit,
    // Tree,
    // Tag,
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

            let size = size.parse::<u64>().context("not a valid blob size ")?;
            let mut z = z.take(size);
            match kind {
                Kind::Blob => {
                    let mut std = std::io::stdout();
                    let mut stdout = std.lock();
                    let n = std::io::copy(&mut z, &mut stdout)
                        .context("writing .git/objects to stdout")?;
                    anyhow::ensure!(
                        n == size,
                        "git object didnt have expected size (expected:{size}, found: {n}"
                    );
                }
            }
        }
        Command::HashObject { write, file } => {
            fn write_blob<W>(file: &Path, writer: W) -> anyhow::Result<String>
            where
                W: Write,
            {
                let stat =
                    std::fs::metadata(&file).with_context(|| format!("stat {}", file.display()))?;
                let writer = ZlibEncoder::new(writer, Compression::default());
                let mut writer = HashWriter {
                    writer,
                    hasher: Sha1::new(),
                };
                write!(writer, "blob ")?;
                write!(writer, "{}\0", stat.len())?;
                let mut file = std::fs::File::open(&file)
                    .with_context(|| format!("open {}", file.display()))?;
                std::io::copy(&mut file, &mut writer).context("stream file into blob")?;
                let _ = writer.writer.finish()?;
                let hash = writer.hasher.finalize();
                Ok(hex::encode(hash))
            }

            let hash = if write {
                let tmp = "temporary";
                let hash = write_blob(
                    &file,
                    std::fs::File::create(tmp).context("construct temporary file for blob")?,
                )
                .context("write out blob object")?;
                fs::create_dir_all(format!(".git/objects/{}/", &hash[..2]))
                    .context("create subdir of .git/objects")?;
                std::fs::rename(tmp, format!(".git/objects/{}/{}", &hash[..2], &hash[2..]))
                    .context("move blob file into .git/objects")?;
                hash
            } else {
                write_blob(&file, std::io::sink()).context("write out blob object")?
            };

            println!("{hash}");
        }
    }

    Ok(())
}

struct HashWriter<W> {
    writer: W,
    hasher: Sha1,
}

impl<W> Write for HashWriter<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let n = self.writer.write(buf)?;
        self.hasher.update(&buf[..n]);
        std::io::Result::Ok(n)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}
