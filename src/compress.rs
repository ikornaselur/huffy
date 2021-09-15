use anyhow::{Context, Result};
use std::fs::File;
use std::io::prelude::*;
use std::io::Bytes;

pub fn compress(file: File) -> Result<()> {
    // Count the occurrances of bytes
    let occ = occurrances(file.bytes());

    Ok(())
}

fn occurrances(byte_iter: Bytes<File>) -> Result<()> {
    for byte in byte_iter {
        println!("{}", byte.unwrap());
    }
    Ok(())
}
