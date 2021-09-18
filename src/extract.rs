use anyhow::Result;
use std::fs::File;

pub fn extract(_file: File) -> Result<()> {
    println!("Extracting");
    Ok(())
}
