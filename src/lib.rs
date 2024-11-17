mod todo;
mod todo_manager;
mod todo_storage;

use thiserror::Error;

pub use todo::Todo;
pub use todo_manager::TodoManager;
pub use todo_storage::TodoStorage;

#[derive(Error, Debug, PartialEq)]
pub enum TodoError {
    #[error("TODO with ID {0} not found!")]
    NotFound(usize),
}
