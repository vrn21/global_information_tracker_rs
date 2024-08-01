use anyhow::Context;
use anyhow::{anyhow, bail, ensure};
use core::fmt;
use flate2::read::ZlibDecoder;
use std::ffi::CStr;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Kind {
    Blob,
    Commit,
    Tree,
    // Tag,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::Blob => write!(f, "blob"),
            Kind::Tree => write!(f, "tree"),
            Kind::Commit => write!(f, "commit"),
        }
    }
}

pub(crate) struct Object<R> {
    pub(crate) kind: Kind,
    pub(crate) expected_size: u64,
    pub(crate) reader: R,
}

impl Object<()> {
    pub(crate) fn read(hash: &str) -> anyhow::Result<(Object<impl BufRead>)> {
        let f = std::fs::File::open(format!(".git/objects/{}/{}", &hash[..2], &hash[2..]))
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
            "tree" => Kind::Tree,
            "commit" => Kind::Commit,
            _ => bail!("we do not yet know how to process  a {kind}"),
        };

        let size = size.parse::<u64>().context("not a valid blob size ")?;
        let mut z = z.take(size);
        Ok(Object {
            kind,
            expected_size: size,
            reader: z,
        })
    }
}
