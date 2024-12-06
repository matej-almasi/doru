//! A simple Todo item.
//!
//! This module contains the `Todo` type and the `TodoStatus` enum representing
//! lifecycle of a `Todo`.
//!
//! # Examples
//!
//! ```
//! use doru::todo::{Todo, TodoStatus};
//!
//! // Create a new Todo item
//! let mut todo = Todo::new(1, "Write module level documentation");
//!
//! assert_eq!(todo.content, "Write module level documentation");
//! assert_eq!(todo.status, TodoStatus::Open);
//!
//! // Change the status of the Todo item
//! todo.status = TodoStatus::InProgress;
//!
//! assert_eq!(todo.status, TodoStatus::InProgress);
//! ```

use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Status of a Todo item.
#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize, ValueEnum)]
pub enum TodoStatus {
    Open,
    InProgress,
    Done,
}

/// A Todo item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Todo {
    id: usize,
    pub content: String,
    pub status: TodoStatus,
}

impl Todo {
    /// Creates a new `Todo` with the given id and content.
    ///
    /// The created `Todo` item starts with a default
    /// [`Open`](TodoStatus::Open) [`TodoStatus`].
    ///
    /// # Examples
    ///
    /// ```
    /// use doru::todo::{Todo, TodoStatus};
    ///
    /// // create a new Todo with Id == 1
    /// let todo = Todo::new(1, "Cook dinner");
    ///
    /// assert_eq!(todo.content, "Cook dinner");
    /// assert_eq!(todo.status, TodoStatus::Open);
    /// assert_eq!(todo.id(), 1);
    /// ```
    pub fn new(id: usize, content: &str) -> Self {
        Self {
            id,
            content: String::from(content),
            status: TodoStatus::Open,
        }
    }

    /// Returns this `Todo`'s id.
    pub fn id(&self) -> usize {
        self.id
    }
}

impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tick = if self.status == TodoStatus::Done {
            "x"
        } else {
            " "
        };

        // Example format: "[x] Learn Rust [Done] (ID: 42)"
        write!(
            f,
            "[{}] {:<20} [{:<12?}] (ID: {})",
            tick, self.content, self.status, self.id
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_id() {
        let todo = Todo::new(42, "Lorem Ipsum");
        assert_eq!(todo.id(), 42);
    }
}
