use sha3::{Digest, Sha3_256};
use std::collections::HashMap;
use std::fs::File;
use std::io::{copy, Read, Write};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HashStorage {
    #[serde(flatten)]
    hash_to_location: HashMap<String, String>,
}

const HASH_LOCATION: &str = "folder_organizer/hashes.json";

impl HashStorage {

    pub fn save_file_location(&mut self, current_file: &str, stored_loc: &str) -> Result<()> {
        let hash = self.get_hash(current_file)?;

        self.hash_to_location.insert(hash, stored_loc.to_string());

        Ok(())
    }

    fn get_hash(&self, file_loc: &str) -> Result<String> {
        let mut cur_file = File::open(file_loc)?;
        let mut hasher = Sha3_256::new();
        copy(&mut cur_file, &mut hasher)?;
        let hash = hasher.finalize();
        let hash_str = hex::encode(hash);
        Ok(hash_str)
    }

    pub fn get_file_loc(&self, current_file: &str) -> Option<&str> {
        let hash = self.get_hash(current_file).map_or(None, |v| Some(v))?;
        self.hash_to_location.get(&hash).map(|owned| &owned[..])
    }

    fn get_existing() -> Result<HashStorage> {
        let mut config_file = File::open(HASH_LOCATION)?;
        let mut contents = Vec::new();
        config_file.read_to_end(&mut contents)?;

        let config: HashStorage = serde_json::from_slice(&contents)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let mut file = File::create(HASH_LOCATION)?;
        file.write_all(serde_json::to_string_pretty(self)?.as_bytes())?;

        Ok(())
    }

    pub fn get_or_default() -> HashStorage {
        let existing = Self::get_existing();

        match existing {
            Ok(config) => config,
            Err(_) => HashStorage {
                hash_to_location: HashMap::new(),
            },
        }
    }
}
