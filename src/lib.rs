mod todo;
mod todo_manager;

use thiserror::Error;

pub use todo::Todo;
pub use todo_manager::TodoManager;

#[derive(Error, Debug, PartialEq)]
pub enum TodoError {
    #[error("TODO with ID {0} not found!")]
    NotFound(usize),
}
