//! # Rudo
//!
//! Rudo is a simple library providing basic TODO functionality. In its heart is
//! a TODO Manager that manages a vector of TODOs - any interaction with
//! individual TODOs is handled by the Manager.
//!
//! Additionally, a TODO Storage trait is provided, defining the contract
//! necessary for loading and storing TODOs from arbitrary source. An example
//! JSON storage implementing the trait is provided.
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

/// Storage of TODO items.
pub mod storage;

/// TODO item structure and related enums.
pub mod todo;

/// TODO manager managing interaction with a collection of TODOs.
pub mod todo_manager;

use thiserror::Error;

/// Enum representing possible errors that can occur while managing TODO items.
#[derive(Error, Debug, PartialEq)]
pub enum TodoError {
    /// Error indicating that a TODO item with the specified ID was not found.
    #[error("TODO with ID {0} not found!")]
    NotFound(usize),
}
