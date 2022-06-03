use anyhow::Result;
use std::{fs, path::Path};

pub fn move_file(source: &Path, dest: &Path) -> Result<()> {
    fs::rename(source, dest)?;
    Ok(())
}
