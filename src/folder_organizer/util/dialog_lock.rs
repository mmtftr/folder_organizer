use anyhow::{anyhow, Result};
use std::fs::{remove_file, File};
use std::io::Write;
use std::path::Path;

const DIALOG_LOCK: &'static str = ".dialog.lock";

pub struct DialogLock {}

impl DialogLock {
  pub fn get_dialog_lock() -> Result<DialogLock> {
    let lock_path = Path::new(DIALOG_LOCK);

    if lock_path.exists() {
      return Err(anyhow!("Dialog lock file exists"));
    }

    let mut file = File::create(lock_path)?;
    file.write_all(b"1")?;

    Ok(DialogLock {})
  }
}

impl Drop for DialogLock {
  fn drop(&mut self) {
    let lock_path = Path::new(DIALOG_LOCK);
    if lock_path.exists() {
      remove_file(lock_path).unwrap();
    }
  }
}
