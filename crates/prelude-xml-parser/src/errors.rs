use std::path::PathBuf;

use thiserror::Error;

/// An enum representing the errors that can occur.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// The file is not a XML file.
    #[error("File {:?} is not a XML file.", 0)]
    InvalidFileType(PathBuf),

    /// The file was not found at the specififed path.
    #[error("File was not found at the specified path: {:?}.", 0)]
    FileNotFound(PathBuf),

    /// An io error occurred.
    #[error(transparent)]
    IO(#[from] std::io::Error),

    /// A parsing error occurred.
    #[error(transparent)]
    ParsingError(#[from] serde_xml_rs::Error),

    /// An unknown error occurred.
    #[error("Unknown error")]
    Unknown,
}
