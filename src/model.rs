use std::{path::{Path, PathBuf}, fs::File, io::Write, collections::HashMap};

use serde::{Deserialize, Serialize};

use crate::{Result, PackwizError};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum HashFormat {
    Sha256,
    Sha512,
    Sha1,
    Md5,
    #[serde(rename = "murmur2")]
    #[default]
    Curseforge,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct Pack {
    /// Path to the folder of the pack.toml file
    #[serde(skip)]
    pub path: PathBuf,

    pub name: String,
    pub author: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub pack_format: String,
    pub index: PackFile,
    pub versions: HashMap<String, String>,
}

impl Pack {
    pub fn load() -> Result<Self> {
        let mut path = std::env::current_dir()?;
        let file = Path::new("pack.toml");

        let found_path = loop {
            path.push(file);

            if path.is_file() {
                break path;
            }

            if !(path.pop() && path.pop()) {
                return Err(PackwizError::PackNotFound);
            }
        };

        Self::load_from(&found_path)
    }

    pub fn load_from(path: &PathBuf) -> Result<Self> {
        let data = std::fs::read_to_string(path)?;
        let mut pack: Self = toml::from_str(&data)?;
        pack.path = path
            .parent()
            .expect("parent folder of pack.toml should be present")
            .to_path_buf();
        Ok(pack)
    }

    pub fn save(&self) -> Result<()> {
        let cfg_str = toml::to_string_pretty(&self)?;
        let mut f = File::create(self.path.join("server.toml"))?;
        f.write_all(cfg_str.as_bytes())?;

        Ok(())
    }

    pub fn get_index(&self) -> Result<PackIndex> {
        let path = self.path.join(&self.index.file);

        PackIndex::load_from(&path)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub struct PackIndex {
    /// Path to the *index* file, not the parent directory
    #[serde(skip)]
    pub path: PathBuf,

    pub hash_format: HashFormat,
    pub files: Vec<PackFile>,
}

impl PackIndex {
    pub fn load_from(path: &PathBuf) -> Result<Self> {
        let data = std::fs::read_to_string(path)?;
        let mut index: Self = toml::from_str(&data)?;
        index.path = path.clone();
        Ok(index)
    }

    pub fn save(&self) -> Result<()> {
        let cfg_str = toml::to_string_pretty(&self)?;
        let mut f = File::create(&self.path)?;
        f.write_all(cfg_str.as_bytes())?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub struct PackFile {
    pub file: String,
    pub hash: String,
    pub hash_format: Option<String>,

    pub alias: Option<String>,
    #[serde(default)]
    pub metafile: bool,
    #[serde(default)]
    pub preserve: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    #[default]
    Both,
    Client,
    Server,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Default)]
#[serde(default)]
pub struct Mod {
    pub name: String,
    pub filename: String,
    pub download: ModDownload,
    pub option: ModOption,
    pub side: Side,
    pub update: Option<ModUpdate>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "kebab-case")]
pub struct ModDownload {
    pub url: Option<String>,
    pub hash: String,
    pub hash_format: HashFormat,
    #[serde(default)]
    pub mode: DownloadMode,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase")]
pub enum DownloadMode {
    #[default]
    #[serde(alias = "")]
    Url,
    #[serde(rename = "metadata:curseforge")]
    Curseforge,
}

// https://github.com/toml-rs/toml/issues/588

/* #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ModUpdate {
    #[serde(rename_all = "kebab-case")]
    Modrinth {
        mod_id: String,
        version: String,
    },
    #[serde(rename_all = "kebab-case")]
    Curseforge {
        file_id: u64,
        project_id: u64,
    },
} */

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct ModUpdate {
    modrinth: Option<ModrinthModUpdate>,
    curseforge: Option<CurseforgeModUpdate>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub struct ModrinthModUpdate {
    mod_id: String,
    version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub struct CurseforgeModUpdate {
    file_id: u64,
    project_id: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Default)]
pub struct ModOption {
    pub optional: bool,
    pub default: bool,
    pub description: Option<String>,
}
