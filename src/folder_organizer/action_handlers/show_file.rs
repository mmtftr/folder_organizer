use anyhow::Result;
use std::{path::Path, process::Command};

pub fn show_file(path: &Path) -> Result<()> {
    Command::new("open")
        .arg("-R")
        .arg(path.to_str().unwrap())
        .spawn()?
        .wait()?;
    Ok(())
}
