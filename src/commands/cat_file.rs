use anyhow::Context;
use anyhow::{anyhow, bail, ensure};
use flate2::read::ZlibDecoder;
use std::ffi::CStr;
use std::io::prelude::*;
use std::io::BufReader;

enum Kind {
    Blob,
    //Commit,
    // Tree,
    // Tag,
}

pub(crate) fn invoke(pretty_print: bool, object_hash: &str) -> anyhow::Result<()> {
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
            let n = std::io::copy(&mut z, &mut stdout).context("writing .git/objects to stdout")?;
            anyhow::ensure!(
                n == size,
                "git object didnt have expected size (expected:{size}, found: {n}"
            );
        }
    }
    Ok(())
}
