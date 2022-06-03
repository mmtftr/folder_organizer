use crate::get_froms::get_froms;
use anyhow::{anyhow, Context, Result};
use serde::Serialize;
use std::{
    env, fs,
    path::{Path, PathBuf},
};
use xattr::XAttrs;

#[derive(Serialize)]
pub struct FileExtras {
    #[serde(skip)]
    pub xattr: XAttrs,
    pub xattr_apple_metadata_kmditemwherefroms: Option<Vec<String>>,
}

#[derive(Serialize, Clone)]
pub struct FilePath {
    full_path: PathBuf,
}

impl FilePath {
    pub fn set_relative_path(&mut self, relative_path: impl AsRef<Path>) -> Result<()> {
        match fs::canonicalize(relative_path) {
            Ok(path) => {
                self.set_full_path(path);
                Ok(())
            }
            Err(e) => Err(anyhow!("Could not canonicalize path: {}", e)),
        }
    }

    pub fn set_full_path(&mut self, new_path: PathBuf) {
        self.full_path = new_path;
    }

    pub fn try_new(full_path: PathBuf) -> Result<Self> {
        Ok(Self {
            full_path: full_path.canonicalize()?,
        })
    }

    pub fn get_relative_path(&self) -> PathBuf {
        self.full_path
            .strip_prefix(&env::current_dir().unwrap())
            .unwrap()
            .to_path_buf()
    }

    pub fn get_relative_path_str(&self) -> String {
        self.get_relative_path()
            .to_str()
            .ok_or_else(|| anyhow!("Could not convert relative path to string"))
            .unwrap()
            .into()
    }
}

#[derive(Serialize)]
pub struct FileInfo {
    pub filepath: FilePath,
    pub name: String,
    pub extras: FileExtras,
    pub deleted: bool,
}

impl FileInfo {
    pub fn try_from(file_path: FilePath) -> Result<Self> {
        Ok(Self {
            filepath: file_path.clone(),
            name: file_path
                .full_path
                .file_name()
                .ok_or(anyhow!("Could not get file name"))?
                .to_str()
                .ok_or(anyhow!("Could not convert file name to str"))?
                .to_string(),
            extras: FileExtras {
                xattr: xattr::list(&file_path.full_path).with_context(|| {
                    format!("Could not get xattrs for file {:?}", file_path.full_path)
                })?,
                xattr_apple_metadata_kmditemwherefroms: file_path
                    .full_path
                    .to_str()
                    .and_then(|path| get_froms(path).ok()),
            },
            deleted: false,
        })
    }
    pub fn try_from_str(file_path: String) -> Result<Self> {
        Self::try_from(FilePath::try_new(PathBuf::from(file_path))?)
    }
}
