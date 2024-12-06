//! A `TodoManager`, responsible for managing interaction with a collection of
//! [`Todo`]s.
//!
//! This module contains the [`TodoManager`] type, providing methods for adding,
//! retrieving and modifying [`Todo`]s in a collection.
//!
//! # Examples
//!
//! ```
//! # fn main() -> Result<(), doru::TodoError> {
//! use doru::todo::TodoStatus;
//! use doru::todo_manager::TodoManager;
//!
//! // Create a new TodoManager.
//! let mut manager = TodoManager::default();
//!
//! // Add a new Todo and store its ID.
//! let id = manager.add_todo("Learn to program");
//! assert_eq!(manager.todo_by_id(id).unwrap().content, "Learn to program");
//!
//! // Change the status of the Todo to InProgress.
//! manager.change_todo_status(id, TodoStatus::InProgress)?;
//! assert_eq!(
//!     manager.todo_by_id(id).unwrap().status,
//!     TodoStatus::InProgress
//! );
//!
//! // Add another Todo and retrieve all Todos.
//! manager.add_todo("Learn Rust");
//! let todos = manager.all_todos();
//! assert_eq!(todos.len(), 2);
//!
//! // Edit the content of the first Todo.
//! manager.edit_todo_content(id, "Learn to program like a Rustacean")?;
//! assert_eq!(
//!     manager.todo_by_id(id).unwrap().content,
//!     "Learn to program like a Rustacean"
//! );
//!
//! // Delete the first Todo.
//! manager.delete_todo(id)?;
//! assert!(manager.todo_by_id(id).is_none());
//! # Ok(())
//! # }
//! ```

use crate::todo::Todo;
use crate::todo::TodoStatus;
use crate::TodoError;

/// A `TodoManager`, responsible for managing interaction with a collection of
/// [`Todo`]s.
///
/// This type provides methods for adding, retrieving and modifying [`Todo`]s in
/// a collection held by the `TodoManager`.
///
/// # Examples
///
/// Create a default `TodoManager` with an empty collection of Todos:
/// ```
/// let mut manager = doru::todo_manager::TodoManager::default();
/// ```
///
/// Add a new Todo to the manager:
/// ```
/// # let mut manager = doru::todo_manager::TodoManager::default();
/// let id = manager.add_todo("Learn Rust");
/// ```
///
/// Change status of the Todo:
/// ```
/// # let mut manager = doru::todo_manager::TodoManager::default();
/// # let id = manager.add_todo("Learn Rust");
/// manager.change_todo_status(id, doru::todo::TodoStatus::InProgress);
/// ```
///
/// Edit the Todo's content:
/// ```
/// # let mut manager = doru::todo_manager::TodoManager::default();
/// # let id = manager.add_todo("Learn Rust");
/// manager.edit_todo_content(id, "Learn Rust like a pro");
/// ```
///
/// Delete the Todo:
/// ```
/// # let mut manager = doru::todo_manager::TodoManager::default();
/// # let id = manager.add_todo("Learn Rust");
/// manager.delete_todo(id);
/// ```
#[derive(Default)]
pub struct TodoManager {
    id_counter: usize,
    todos: Vec<Todo>,
}

impl TodoManager {
    /// Creates a new `TodoManager` holding the provided list of [`Todo`]s.
    ///
    /// # Examples
    ///
    /// ```
    /// # use doru::todo::Todo;
    /// # use doru::todo_manager::TodoManager;
    /// let todos = vec![Todo::new(0, "Learn Rust"), Todo::new(1, "Learn to cook")];
    /// let manager = TodoManager::new(todos.clone());
    ///
    /// assert_eq!(manager.todo_by_id(0).unwrap(), &todos[0]);
    /// assert_eq!(manager.todo_by_id(1).unwrap(), &todos[1]);
    /// ```
    pub fn new(todos: Vec<Todo>) -> Self {
        let last_id = todos.iter().map(|todo| todo.id()).max().unwrap_or(0);

        Self {
            id_counter: last_id,
            todos,
        }
    }

    /// Creates a new [`Todo`] with the provided content and stores it
    /// internally, then returns id of the newly created [`Todo`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use doru::todo_manager::TodoManager;
    /// let mut manager = TodoManager::default();
    /// let id = manager.add_todo("Learn Rust");
    ///
    /// assert_eq!(manager.todo_by_id(id).unwrap().content, "Learn Rust");
    /// ```
    pub fn add_todo(&mut self, content: &str) -> usize {
        self.id_counter += 1;
        self.todos.push(Todo::new(self.id_counter, content));

        self.todos.last().unwrap().id()
    }

    /// Returns a [`Vec`] of references to all internally stored [`Todo`]s
    ///
    /// # Examples
    ///
    /// ```
    /// # use doru::todo_manager::TodoManager;
    /// let mut manager = TodoManager::default();
    /// manager.add_todo("Learn Rust");
    /// manager.add_todo("Learn to cook");
    /// manager.add_todo("Learn to program");
    /// manager.add_todo("Learn to dance");
    ///
    /// let todos = manager.all_todos();
    ///
    /// assert_eq!(todos.len(), 4);
    /// ```
    pub fn all_todos(&self) -> Vec<&Todo> {
        self.todos.iter().collect()
    }

    /// Returns a reference to a [`Todo`] with the provided id, if it exists,
    /// otherwise returns [`None`]
    ///
    /// # Examples
    ///
    /// ```
    /// # use doru::todo_manager::TodoManager;
    /// let mut manager = TodoManager::default();
    /// let id = manager.add_todo("Learn Rust");
    ///
    /// // Retrieve a Todo that exists
    /// let todo = manager.todo_by_id(id);
    /// assert_eq!(todo.unwrap().content, "Learn Rust");
    ///
    /// // Retrieve a Todo that doesn't exist
    /// let non_existent_todo = manager.todo_by_id(42);
    /// assert!(non_existent_todo.is_none());
    /// ```
    pub fn todo_by_id(&self, id: usize) -> Option<&Todo> {
        self.todos.iter().find(|todo| todo.id() == id)
    }

    /// Returns a [`Vec`] of references to all [`Todo`]s that have the provided
    /// [`TodoStatus`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use doru::todo::TodoStatus;
    /// # use doru::todo_manager::TodoManager;
    /// let mut manager = TodoManager::default();
    /// manager.add_todo("Learn Rust");
    /// manager.add_todo("Learn to cook");
    /// manager.add_todo("Learn to program");
    /// manager.add_todo("Learn to dance");
    ///
    /// manager.change_todo_status(1, TodoStatus::InProgress);
    /// manager.change_todo_status(3, TodoStatus::Done);
    ///
    /// let open_todos = manager.todos_by_status(TodoStatus::Open);
    /// assert_eq!(open_todos.len(), 2);
    ///
    /// let in_progress = manager.todos_by_status(TodoStatus::InProgress);
    /// assert_eq!(in_progress.len(), 1);
    ///
    /// let done = manager.todos_by_status(TodoStatus::Done);
    /// assert_eq!(done.len(), 1);
    /// ```
    pub fn todos_by_status(&self, status: TodoStatus) -> Vec<&Todo> {
        self.todos
            .iter()
            .filter(|todo| todo.status == status)
            .collect()
    }

    /// Changes the content of a [`Todo`] with the provided id.
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if no [`Todo`] with provided id exists.
    ///
    /// # Examples
    ///
    /// ```
    /// # use doru::todo_manager::TodoManager;
    /// let mut manager = TodoManager::default();
    /// let id = manager.add_todo("Learn Rust");
    ///
    /// // Edit content of an existing Todo
    /// manager.edit_todo_content(id, "Learn Rust like a pro");
    ///
    /// // Try to edit content of a non-existent Todo
    /// let result = manager.edit_todo_content(42, "This won't work");
    /// assert!(result.is_err());
    /// ```
    pub fn edit_todo_content(&mut self, id: usize, content: &str) -> Result<(), TodoError> {
        let todo = self.todos.iter_mut().find(|todo| todo.id() == id);

        if let Some(todo) = todo {
            todo.content = String::from(content);
            Ok(())
        } else {
            Err(TodoError::NotFound(id))
        }
    }

    /// Changes the status of a [`Todo`] with the provided id.
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if no [`Todo`] with provided id exists.
    ///
    /// # Examples
    ///
    /// ```
    /// # use doru::todo::TodoStatus;
    /// # use doru::todo_manager::TodoManager;
    /// let mut manager = TodoManager::default();
    /// let id = manager.add_todo("Learn Rust");
    ///
    /// // Change status of an existing Todo
    /// manager.change_todo_status(id, TodoStatus::InProgress);
    /// assert_eq!(
    ///     manager.todo_by_id(id).unwrap().status,
    ///     TodoStatus::InProgress
    /// );
    ///
    /// // Try to change status of a non-existent Todo
    /// let result = manager.change_todo_status(42, TodoStatus::Done);
    /// assert!(result.is_err());
    /// ```
    pub fn change_todo_status(&mut self, id: usize, state: TodoStatus) -> Result<(), TodoError> {
        let todo = self.todos.iter_mut().find(|todo| todo.id() == id);

        if let Some(todo) = todo {
            todo.status = state;
            Ok(())
        } else {
            Err(TodoError::NotFound(id))
        }
    }

    /// Deletes a [`Todo`] with the provided id.
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if no [`Todo`] with provided id exists.
    ///
    /// # Examples
    ///
    /// ```
    /// # use doru::todo_manager::TodoManager;
    /// let mut manager = TodoManager::default();
    /// let id = manager.add_todo("Learn Rust");
    ///
    /// // Delete an existing Todo
    /// manager.delete_todo(id);
    /// assert!(manager.todo_by_id(id).is_none());
    ///
    /// // Try to delete a non-existent Todo
    /// let result = manager.delete_todo(42);
    /// assert!(result.is_err());
    /// ```
    pub fn delete_todo(&mut self, id: usize) -> Result<(), TodoError> {
        let todo_position = self.todos.iter().position(|todo| todo.id() == id);

        if let Some(position) = todo_position {
            self.todos.remove(position);
            Ok(())
        } else {
            Err(TodoError::NotFound(id))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_from_empty_vec_is_empty() {
        let manager = TodoManager::new(vec![]);

        assert_eq!(manager.todos, vec![]);
    }

    #[test]
    fn new_from_empty_has_last_counter_0() {
        let manager = TodoManager::new(vec![]);

        assert_eq!(manager.id_counter, 0);
    }

    #[test]
    fn new_from_existing_is_filled() {
        let todos = vec![Todo::new(0, "Lorem"), Todo::new(1, "Ipsum")];

        let manager = TodoManager::new(todos.clone());

        assert_eq!(manager.todos, todos);
    }

    #[test]
    fn new_from_existing_has_correct_id_counter() {
        let todos = vec![
            Todo::new(0, "Lorem"),
            Todo::new(10, "Ipsum"),
            Todo::new(2, "Dolor"),
        ];

        let manager = TodoManager::new(todos.clone());

        assert_eq!(manager.id_counter, 10);
    }

    #[test]
    fn add_todo_adds_todo() {
        let mut manager = TodoManager::default();

        let content = "Lorem Ipsum";
        manager.add_todo(content);

        assert_eq!(manager.todos.len(), 1);
        assert_eq!(manager.todos[0].content, content);
        assert_eq!(manager.todos[0].status, TodoStatus::Open)
    }

    #[test]
    fn add_todo_returns_correct_id() {
        let mut manager = TodoManager::default();

        let new_id = manager.add_todo("content");
        assert_eq!(new_id, 1);

        let new_id = manager.add_todo("another");
        assert_eq!(new_id, 2);
    }

    #[test]
    fn ids_increment() {
        let mut manager = TodoManager::default();

        manager.add_todo("Lorem");
        manager.add_todo("Ipsum");

        assert_eq!(manager.todos[1].id(), 2);
    }

    #[test]
    fn get_open_todos() {
        let mut manager = TodoManager::default();
        manager.add_todo("Lorem");
        manager.add_todo("Ipsum");
        manager.add_todo("Dolor");

        manager.todos[2].status = TodoStatus::InProgress;

        let open_todos = manager.todos_by_status(TodoStatus::Open);

        assert_eq!(open_todos.len(), 2);

        assert_eq!(open_todos, vec![&manager.todos[0], &manager.todos[1]]);
    }

    #[test]
    fn get_in_progress_todos() {
        let mut manager = TodoManager::default();
        manager.add_todo("Lorem");
        manager.add_todo("Ipsum");
        manager.add_todo("Dolor");

        manager.todos[1].status = TodoStatus::InProgress;

        let in_progress = manager.todos_by_status(TodoStatus::InProgress);

        assert_eq!(in_progress.len(), 1);

        assert_eq!(in_progress, vec![&manager.todos[1]]);
    }

    #[test]
    fn get_all_todos() {
        let mut manager = TodoManager::default();

        manager.add_todo("Lorem");
        manager.add_todo("Ipsum");
        manager.add_todo("Dolor");
        manager.add_todo("Sit");

        let todos = manager.all_todos();

        assert_eq!(todos.len(), 4)
    }

    #[test]
    fn get_existing_todo_by_id() {
        let mut manager = TodoManager::default();

        manager.add_todo("This has id 1");
        manager.add_todo("This has id 2");
        let id = manager.add_todo("This has id 3");

        let todo = manager.todo_by_id(id);

        assert_eq!(todo, Some(&manager.todos[2]))
    }

    #[test]
    fn try_getting_non_existent_todo_by_id() {
        let manager = TodoManager::default();

        let todo = manager.todo_by_id(42);

        assert!(todo.is_none())
    }

    #[test]
    fn edit_existing_todo_content_succeeds() {
        let mut manager = TodoManager::default();

        let new_id = manager.add_todo("This is a nice TODO.");

        let test_content = "This is even better!";
        let result = manager.edit_todo_content(new_id, test_content);

        assert_eq!(result, Ok(()));

        let updated_content = &manager.todo_by_id(new_id).unwrap().content;
        assert_eq!(updated_content, test_content)
    }

    #[test]
    fn edit_nonexistent_todo_content_fails() {
        let mut manager = TodoManager::default();

        let result = manager.edit_todo_content(1, "Some content.");
        assert_eq!(result, Err(TodoError::NotFound(1)))
    }

    #[test]
    fn change_existing_todo_status_succeeds() {
        let mut manager = TodoManager::default();
        let new_id = manager.add_todo("Good to do.");

        let new_state = TodoStatus::InProgress;

        let result = manager.change_todo_status(new_id, new_state);
        assert_eq!(result, Ok(()));

        let updated_state = &manager.todo_by_id(new_id).unwrap().status;
        assert_eq!(*updated_state, new_state);
    }

    #[test]
    fn change_nonexistent_todo_status_fails() {
        let mut manager = TodoManager::default();

        let result = manager.change_todo_status(42, TodoStatus::Done);
        assert_eq!(result, Err(TodoError::NotFound(42)));
    }

    #[test]
    fn delete_existing_todo_succeeds() {
        let mut manager = TodoManager::default();
        let new_id = manager.add_todo("Lorem Ipsum");

        let result = manager.delete_todo(new_id);
        assert_eq!(result, Ok(()));

        assert!(manager.todos.is_empty());
    }

    #[test]
    fn delete_nonexistent_todo_fails() {
        let mut manager = TodoManager::default();

        let result = manager.delete_todo(42);
        assert_eq!(result, Err(TodoError::NotFound(42)));
    }
}
