use crate::folder_organizer::config::OrganizerConfig;

use super::{file::FileInfo, file_action::FileAction};

pub trait Matcher {
    fn match_action(&self, file: &FileInfo, config: &OrganizerConfig) -> FileAction;
}
