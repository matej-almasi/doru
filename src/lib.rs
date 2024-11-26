pub mod storage;
pub mod todo;
pub mod todo_manager;

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum TodoError {
    #[error("TODO with ID {0} not found!")]
    NotFound(usize),
}
