use std::path::PathBuf;

use crate::folder_organizer::{
    config::OrganizerConfig,
    util::{file::FileInfo, file_action::FileAction, matcher::Matcher},
};

pub struct HashStorageMatcher();

impl HashStorageMatcher {
    pub fn new() -> Self {
        Self {}
    }
    pub fn find_stored_loc(
        &self,
        file_info: &FileInfo,
        config: &OrganizerConfig,
    ) -> Option<PathBuf> {
        let potential_loc = config
            .hash_storage
            .get_file_loc(&file_info.filepath.get_relative_path_str());

        potential_loc.map(|path| PathBuf::from(path))
    }
}

impl Matcher for HashStorageMatcher {
    fn match_action(&self, file_info: &FileInfo, config: &OrganizerConfig) -> FileAction {
        if let Some(new_path) = self.find_stored_loc(file_info, config) {
            return FileAction::Multiple(vec![FileAction::Move(new_path), FileAction::Show(None)]);
        }
        return FileAction::Fallthrough;
    }
}

pub const MATCHER_NAME: &str = "hash_storage";
