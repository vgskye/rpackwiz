use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum HashFormat {
    Sha256,
    Sha512,
    Sha1,
    Md5,
    #[serde(rename = "murmur2")]
    Curseforge,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub struct PackFile {
    pub index: PackFileIndex,
    pub name: String,
    pub pack_format: String,
    pub versions: PackFileVersions,
    pub author: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub struct PackFileIndex {
    pub file: String,
    pub hash: String,
    pub hash_format: HashFormat,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct PackFileVersions {
    pub minecraft: String,
    pub fabric: Option<String>,
    pub forge: Option<String>,
    pub liteloader: Option<String>,
    pub quilt: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub struct IndexFile {
    pub hash_format: HashFormat,
    pub files: Vec<IndexFileEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub struct IndexFileEntry {
    pub file: String,
    pub alias: Option<String>,
    pub hash: String,
    pub hash_format: Option<String>,
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct MetaFile {
    pub download: MetaFileDownload,
    pub filename: String,
    pub name: String,
    #[serde(default)]
    pub option: MetaFileOptionalState,
    #[serde(default)]
    pub side: Side,
    #[serde(default)]
    pub update: MetaFileUpdateState,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub struct MetaFileDownload {
    pub hash: String,
    pub hash_format: HashFormat,
    #[serde(default)]
    pub mode: MetaFileDownloadMode,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase")]
pub enum MetaFileDownloadMode {
    #[default]
    #[serde(alias = "")]
    Url,
    #[serde(rename = "metadata:curseforge")]
    Curseforge,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Default)]
pub struct MetaFileOptionalState {
    pub optional: bool,
    pub default: bool,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub struct MetaFileUpdateState {
    pub curseforge: Option<MetaFileUpdateStateCurseforge>,
    pub modrinth: Option<MetaFileUpdateStateModrinth>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub struct MetaFileUpdateStateCurseforge {
    file_id: u64,
    project_id: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub struct MetaFileUpdateStateModrinth {
    mod_id: String,
    version: String,
}
