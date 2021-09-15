use anyhow::{Context, Result};
use std::fs::File;

pub fn compress(file: File) -> Result<()> {
    println!("Compressing");
    Ok(())
}
