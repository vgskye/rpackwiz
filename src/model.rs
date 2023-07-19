use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub struct Pack {
    pub name: String,
    pub author: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub pack_format: String,
    pub index: PackFile,
    pub versions: PackDependencies,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct PackDependencies {
    pub minecraft: String,
    pub fabric: Option<String>,
    pub forge: Option<String>,
    pub liteloader: Option<String>,
    pub quilt: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub struct PackIndex {
    pub hash_format: HashFormat,
    pub files: Vec<PackFile>,
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
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
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Default)]
pub struct ModOption {
    pub optional: bool,
    pub default: bool,
    pub description: Option<String>,
}
