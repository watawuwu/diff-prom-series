use anyhow::Result;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

pub fn read_file(path: impl AsRef<Path>) -> Result<Vec<u8>> {
    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}
