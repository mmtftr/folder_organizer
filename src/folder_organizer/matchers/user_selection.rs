use std::path::PathBuf;

use native_dialog::{FileDialog, MessageDialog, MessageType};

use crate::folder_organizer::{
    config::OrganizerConfig,
    util::{dialog_lock::DialogLock, file::FileInfo, file_action::FileAction, matcher::Matcher},
};

pub struct UserSelectionMatcher();

impl UserSelectionMatcher {
    pub fn new() -> Self {
        Self {}
    }
    pub fn prompt_process(
        &self,
        file_info: &FileInfo,
        config: &OrganizerConfig,
    ) -> Option<PathBuf> {
        let dialog_lock = DialogLock::get_dialog_lock();
        if dialog_lock.is_err() {
            return None;
        }

        let filename = file_info.name.clone();
        let initial_location = config.get_choose_folder();

        let ask_user_continue = MessageDialog::new()
            .set_type(MessageType::Info)
            .set_title("Folder Organizer: Decide Action")
            .set_text(&format!(
                "Could not find a proper location for file {}. Would you like to choose a folder?",
                filename
            ))
            .show_confirm();

        if let Ok(true) = ask_user_continue {
            let picked_folder = FileDialog::new()
                .set_location(&initial_location)
                .show_open_single_dir();
            if let Ok(Some(folder)) = picked_folder {
                return Some(folder);
            }
        }

        None
    }
}

impl Matcher for UserSelectionMatcher {
    fn match_action(&self, file_info: &FileInfo, config: &OrganizerConfig) -> FileAction {
        if let Some(new_path) = self.prompt_process(file_info, config) {
            return FileAction::Multiple(vec![FileAction::Move(new_path), FileAction::Show(None)]);
        }
        return FileAction::Fallthrough;
    }
}

pub const MATCHER_NAME: &str = "user_selection";
