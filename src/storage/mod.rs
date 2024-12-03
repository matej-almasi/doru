//! File storage for `Todo`s.
//!
//! This module contains the [`TodoStorage`] trait that defines the contract for
//! loading and storing `Todo`s from/ to arbitrary text format. An example
//! [`JsonStorage`] type implementing the trait is provided.

mod json_storage;
mod todo_storage;

pub use json_storage::JsonStorage;
pub use todo_storage::{TodoStorage, TodoStorageError};
