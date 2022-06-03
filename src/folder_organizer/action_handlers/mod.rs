mod move_file;
mod show_file;

use std::{
    fs::{self, canonicalize},
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};

use super::{
    config::OrganizerConfig,
    util::{file::FileInfo, file_action::FileAction},
};

pub fn ensure_loc_safe(to_loc: &Path, base_dir: &Path) -> Result<()> {
    if canonicalize(
        to_loc
            .parent()
            .ok_or(anyhow!("Relative path does not have parent"))?,
    )
    .unwrap()
        == canonicalize(base_dir).unwrap()
    {
        log::error!(
            "Destination directory is the same as the base directory. Move location: {}",
            to_loc.to_str().unwrap()
        );
        anyhow!(
            "Destination directory is the same as the base directory, {:?} == {:?}",
            to_loc,
            base_dir
        );
    }
    Ok(())
}

pub fn execute_action(
    file: &mut FileInfo,
    action: FileAction,
    config: &OrganizerConfig,
) -> Result<()> {
    if file.deleted {
        return Ok(());
    }
    use crate::folder_organizer::util::file_action::FileAction::*;
    match action {
        Move(new_loc) => {
            if config.verify_new_loc {
                let is_loc_safe = ensure_loc_safe(&new_loc, PathBuf::from(".").as_path());
                if is_loc_safe.is_err() {
                    log::error!("Unsafe move location {:?} in the same directory. Might cause a recursive folder action call, thus aborting. Set 'verify_new_loc' to false to disable this check.", new_loc);
                    return is_loc_safe;
                }
            }
            log::info!(
                "Executing action Move to move {} to {}",
                file.filepath.get_relative_path_str(),
                new_loc.to_str().unwrap()
            );
            move_file::move_file(&file.filepath.get_relative_path(), &new_loc)?;

            file.filepath.set_relative_path(new_loc)?;
        }
        Show(to_loc) => {
            log::info!(
                "Executing action Show to show {}",
                file.filepath.get_relative_path_str()
            );
            show_file::show_file(&match to_loc {
                Some(loc) => loc,
                None => file.filepath.get_relative_path(),
            })?;
        }
        Delete => {
            log::info!(
                "Executing action Delete to delete {}",
                file.filepath.get_relative_path_str()
            );
            fs::remove_file(file.filepath.get_relative_path())?;
            file.deleted = true;
        }
        Multiple(actions) => {
            for action in actions {
                execute_action(file, action, config)?;
            }
        }
        Noop | Fallthrough => {
            log::info!(
                "Executing default action (ignoring) to file {}",
                file.filepath.get_relative_path_str()
            );
        }
    }
    Ok(())
}
