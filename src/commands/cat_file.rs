use crate::objects::{self, Kind, Object};
use anyhow::ensure;
use anyhow::Context;

pub(crate) fn invoke(pretty_print: bool, object_hash: &str) -> anyhow::Result<()> {
    ensure!(pretty_print, "pretty print must be given (-p)");

    let mut object = Object::read(object_hash).context("parse out object file")?;
    match object.kind {
        Kind::Blob => {
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
