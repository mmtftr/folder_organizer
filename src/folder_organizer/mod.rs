mod action_handlers;
mod config;
mod hash_storage;
mod matchers;
pub mod util;

use std::path::PathBuf;

use anyhow::Result;

use crate::folder_organizer::action_handlers::execute_action;

use self::config::OrganizerConfig;
use self::util::file::FileInfo;
use self::util::file_action::FileAction;

pub struct FolderOrganizer {
    config: OrganizerConfig,
}

impl FolderOrganizer {
    fn decide_file_action(&self, file_info: &FileInfo) -> FileAction {
        let matchers = self
            .config
            .get_matchers_for_file(&file_info.filepath.get_relative_path_str());
        log::debug!(
            "Matchers for file {:?}: {:?}",
            file_info.filepath.get_relative_path_str(),
            matchers.len()
        );

        for matcher in matchers {
            let action = matcher.match_action(file_info, &self.config);

            if let FileAction::Fallthrough = action {
                continue;
            }

            return action;
        }

        FileAction::Fallthrough
    }

    fn organize(&mut self, files: Vec<FileInfo>) -> Result<()> {
        for mut file in files {
            let action = self.decide_file_action(&file);
            execute_action(&mut file, action, &self.config)?;
            if self.config.store_hash_location_map {
                let path = file.filepath.get_relative_path_str();
                self.config.hash_storage.save_file_location(&path, &path)?;
            }
        }
        Ok(())
    }

    pub fn organize_files(&mut self, files: Vec<String>) -> Result<()> {
        let files = files
            .into_iter()
            .filter_map(|path| FileInfo::try_from_str(path).ok())
            .collect::<Vec<_>>();
        self.organize(files)
    }

    pub fn save_data(&self) -> Result<()> {
        self.config.save()?;
        self.config.hash_storage.save()?;

        Ok(())
    }

    pub fn new() -> FolderOrganizer {
        let conf = OrganizerConfig::get_or_default();
        FolderOrganizer { config: conf }
    }
}
