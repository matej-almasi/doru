use crate::{Todo, TodoError, TodoManager};

pub trait TodoStorage {
    fn load() -> Result<TodoManager, TodoError>;
    fn save(todos: &[Todo]) -> Result<(), TodoError>;
}
