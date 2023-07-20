pub mod hash;
pub mod model;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum PackwizError {
    #[error("IO Error")]
    IOError(#[from] std::io::Error),
    #[error("TOML Deserialize Error")]
    DeserializeError(#[from] toml::de::Error),
    #[error("TOML Serialize Error")]
    SerializeError(#[from] toml::ser::Error),
    #[error("pack.toml file was not found, try packwiz init?")]
    PackNotFound,
}

type Result<T> = core::result::Result<T, PackwizError>;
