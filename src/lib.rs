//! # Rudo
//!
//! Rudo is a simple library providing basic `Todo` functionality. In its heart
//! is a `TodoManager` that manages a vector of `Todo`s - any interaction with
//! individual `Todo`s is handled by the Manager.
//!
//! Additionally, a `TodoStorage` trait defines the contract for loading and
//! storing `Todo`s from/ to arbitrary text format. An example JSON storage
//! implementing the trait is provided.
//!
//! In some cases, the operations can fail. The `TodoError` enum defines the
//! possible errors.
//!
//! # Example
//!
//! ```
//! use rudo::todo::TodoStatus;
//! use rudo::todo_manager::TodoManager;
//!
//! let mut manager = TodoManager::default();
//!
//! let id = manager.add_todo("Learn Rust");
//! manager.change_status(id, TodoStatus::InProgress).unwrap();
//!
//! let todos = manager.get_all();
//!
//! for todo in &todos {
//!     println!("{:?}", todo);
//! }
//! ```

pub mod storage;

pub mod todo;

pub mod todo_manager;

use thiserror::Error;

/// Possible errors that can occur while managing Todo items.
#[derive(Error, Debug, PartialEq)]
pub enum TodoError {
    /// Error indicating that a Todo item with the specified ID was not found.
    #[error("Todo with ID {0} not found!")]
    NotFound(usize),
}
