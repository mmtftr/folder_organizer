use anyhow::Result;
use regex::Regex;
use std::io::Write;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::hash_storage::HashStorage;
use std::{fs::File, io::Read};
#[derive(Serialize, Deserialize, Debug)]
pub struct MatcherSelector {
    #[serde(with = "serde_regex")]
    pub regex: Regex,
    pub name: String,
    pub options: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrganizerConfig {
    #[serde(default)]
    pub dialog_folder: String,
    #[serde(default)]
    pub matchers: Vec<MatcherSelector>,
    // Stores a map of hash -> file location
    #[serde(default = "default_hash_location_map")]
    pub store_hash_location_map: bool,
    // Verifies that the move destination is a different folder than the source
    // This is to prevent recursion hell if the use case is Folder Actions
    #[serde(default = "default_verify_new_loc")]
    pub verify_new_loc: bool,
    #[serde(skip, default = "HashStorage::get_or_default")]
    pub hash_storage: HashStorage,
}

impl Default for OrganizerConfig {
    fn default() -> Self {
        OrganizerConfig {
            dialog_folder: String::from("."),
            matchers: vec![],
            store_hash_location_map: true,
            verify_new_loc: true,
            hash_storage: HashStorage::get_or_default(),
        }
    }
}

pub fn default_hash_location_map() -> bool {
    true
}

pub fn default_verify_new_loc() -> bool {
    true
}

const CONFIG_PATH: &str = "folder_organizer/config.json";

use super::{matchers::get_matcher_from_name, util::matcher::Matcher};

impl OrganizerConfig {
    fn get_existing() -> Result<OrganizerConfig> {
        let mut config_file = File::open(CONFIG_PATH)?;
        let mut contents = Vec::new();
        config_file.read_to_end(&mut contents)?;

        let config: OrganizerConfig = serde_json::from_slice(&contents)?;
        Ok(config)
    }

    pub fn get_matchers_for_file(&self, file_name: impl AsRef<str>) -> Vec<Box<dyn Matcher>> {
        let mut matchers = Vec::new();
        for matcher in &self.matchers {
            if matcher.regex.is_match(file_name.as_ref()) {
                let matcher_name = &matcher.name;
                let matcher = get_matcher_from_name(matcher_name, &matcher.options);
                if let Ok(matcher) = matcher {
                    matchers.push(matcher);
                }
            }
        }
        matchers
    }

    pub fn save(&self) -> Result<()> {
        let mut file = File::create(CONFIG_PATH)?;
        file.write_all(serde_json::to_string_pretty(self)?.as_bytes())?;

        Ok(())
    }

    pub fn get_choose_folder(&self) -> PathBuf {
        PathBuf::from(&self.dialog_folder)
    }

    pub fn get_or_default() -> OrganizerConfig {
        let existing = Self::get_existing();

        match existing {
            Ok(config) => config,
            Err(_) => OrganizerConfig::default(),
        }
    }
}
