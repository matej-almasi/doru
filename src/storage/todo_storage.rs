use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::todo::Todo;

/// Trait defining the contract for loading and storing [`Todo`]s from/ to
/// files with arbitrary text format.
pub trait TodoStorage {
    /// Loads [`Todo`]s from a file at the given path.
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if the operation fails. This can be due to various
    /// reasons, such as the file not existing, not having the necessary
    /// permissions, or the file not being in the expected format.
    fn load(path: &Path) -> Result<Vec<Todo>, TodoStorageError>;

    /// Saves the given [`Todo`]s to a file at the given path.
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if the operation fails. This can be due to various
    /// reasons, such as the file not existing, not having the necessary
    /// permissions, or the data not being serializable.
    fn save(todos: &[&Todo], path: &Path) -> Result<(), TodoStorageError>;
}

/// Possible errors that can occur while loading or storing [`Todo`] items.
#[derive(Error, Debug, PartialEq)]
pub enum TodoStorageError {
    /// Error interacting with a file at the specified path.
    #[error("Failed operation with file {0}!")]
    FileError(PathBuf),

    /// Error parsing data from a file.
    #[error("Failed parsing {0}!")]
    ParseError(PathBuf),

    /// Error serializing data.
    #[error("Failed to serialize data!")]
    SerializeError,
}
