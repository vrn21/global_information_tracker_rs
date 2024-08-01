use std::ffi::CStr;
use std::io::BufRead;
use std::io::Read;
use std::io::Write;

use crate::objects::{Kind, Object};
use anyhow::Context;
use flate2::write;

pub(crate) fn invoke(name_only: bool, tree_hash: &str) -> anyhow::Result<()> {
    let mut object = Object::read(tree_hash).context("parse out object file")?;
    match object.kind {
        Kind::Tree => {
            let mut buf = Vec::new();
            let mut stdout = std::io::stdout();
            let mut hashbuf = Vec::new();

            loop {
                buf.clear();
                let n = object
                    .reader
                    .read_until(0, &mut buf)
                    .context("reading tree entries")?;

                if n == 0 {
                    break;
                }

                object
                    .reader
                    .read_exact(&mut hashbuf[..])
                    .context("read tree object hash")?;

                let mode_and_name =
                    CStr::from_bytes_with_nul(&buf).context("invalid tree enrty")?;
                let mut bits = mode_and_name.to_bytes().splitn(2, |&b| b == b' ');
                let mode = bits.next().expect("split always yields next");
                let name = bits
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("tree entry has no filename"))?;
                let hash = hex::encode(&hashbuf);

                if name_only {
                    stdout
                        .write_all(name)
                        .context("write tree name to stdout")?;
                } else {
                    let mode = std::str::from_utf8(mode).context("it is alwasy utf8")?;
                    stdout
                        .write_all(mode)
                        .context("write tree name to stdout")?;
                    let hash = hex::encode(&hashbuf);
                    let object =
                        Object::read(&hash).with_context(|| format!("read object for tree {hash}"));
                    //let kind = "tree";
                    write!(stdout, " {mode:0>06} {kind} {hash} ")
                        .context("write tree hash to stdout ")?;
                    stdout
                        .write_all(name)
                        .context("write tree name to stdout")?;
                }

                writeln!(stdout, "").context("writing to stdout")?;
            }
            let mut std = std::io::stdout();
            let mut stdout = std.lock();
            let n = std::io::copy(&mut object.reader, &mut stdout)
                .context("writing .git/objects to stdout")?;
            anyhow::ensure!(
                n == object.expected_size,
                format!(
                    "git object didnt have expected size (expected:{}, found: {}",
                    object.expected_size, n
                )
            );
        }
        _ => anyhow::bail!("dont know how to print anything else: {}", object.kind),
    }
    Ok(())
}
