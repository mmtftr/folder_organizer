use crate::folder_organizer::PathBuf;
use serde::{Deserialize, Serialize};

// Enum that determines what to do with an existing file
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action", content = "args", rename_all = "camelCase")]
pub enum FileAction {
    // Move the file to another location
    Move(PathBuf),
    // Show the file in question(None) or another path(Some)
    Show(Option<PathBuf>),
    // After delete we stop executing actions since the file no longer exists.
    // If you want to do anything else, do it before deleting the file.
    Delete,
    // We always assume that Multiple contains non-Fallthrough actions.
    // This means that vec![FileAction::Fallthrough] will *NOT* fall through.
    Multiple(Vec<FileAction>),
    // Noop means that no action should be executed
    Noop,
    // Fallthrough means that next matchers in the list should be executed
    Fallthrough,
}
