use crate::archive::{ArchiveID, AssetID};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AssetLookupError {
    #[error("Asset lookup failed, there is no asset at \"{1}\" in archive {0}")]
    NotFoundPath(ArchiveID, String),

    #[error("Asset lookup failed, there is no asset with the id {1} in archive {0}")]
    NotFoundID(ArchiveID, AssetID),

    #[error("Asset lookup failed due to an unknown reason. Implementation message: \"{0}\"")]
    Unknown(ArchiveID, String),
}

#[derive(Error, Debug)]
pub enum AssetInsertError {
    #[error("Asset insertion failed, there is already an asset at \"{1}\" in archive {0}")]
    AssetAlreadyExistsPath(ArchiveID, String),

    #[error("Asset insertion failed, there is already an asset with the id {1} in archive {0}")]
    AssetAlreadyExistsID(ArchiveID, AssetID),

    #[error("Asset insert failed due to an unknown reason. Implementation message: \"{0}\"")]
    Unknown(ArchiveID, String),
}

#[derive(Error, Debug)]
pub enum AssetRemoveError {
    #[error(transparent)]
    LookupError(#[from] AssetLookupError),

    #[error("Asset removal failed due to an unknown reason in archive {0}. Implementation message: \"{1}\"")]
    Unknown(ArchiveID, String),
}
