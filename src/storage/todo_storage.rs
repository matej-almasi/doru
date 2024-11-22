use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::Todo;

pub trait TodoStorage {
    fn load(path: &Path) -> Result<Vec<Todo>, TodoStorageError>;
    // fn save(todos: &[Todo], path: &Path) -> Result<(), TodoStorageError>;
}

#[derive(Error, Debug, PartialEq)]
pub enum TodoStorageError {
    #[error("Failed reading from {0}!")]
    FileError(PathBuf),

    #[error("Failed parsing {0}!")]
    ParseError(PathBuf),
}