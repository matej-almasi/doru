use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum TodoError {
    #[error("TODO with ID {0} not found!")]
    NotFound(usize),
}

#[derive(Default)]
pub struct TodoManager {
    id_counter: usize,
    todos: Vec<Todo>,
}

impl TodoManager {
    pub fn add_todo(&mut self, content: &str) -> usize {
        self.id_counter += 1;
        self.todos.push(Todo::new(self.id_counter, content));

        self.todos.last().unwrap().id
    }

    pub fn get_all(&self) -> Vec<&Todo> {
        self.todos.iter().collect()
    }

    pub fn get_by_id(&self, id: usize) -> Option<&Todo> {
        self.todos.iter().find(|todo| todo.id == id)
    }

    pub fn get_by_state(&self, state: TodoState) -> Vec<&Todo> {
        self.todos
            .iter()
            .filter(|todo| todo.state == state)
            .collect()
    }

    pub fn edit_content(&mut self, id: usize, content: &str) -> Result<(), TodoError> {
        let todo = self.todos.iter_mut().find(|todo| todo.id == id);

        if let Some(todo) = todo {
            todo.content = String::from(content);
            Ok(())
        } else {
            Err(TodoError::NotFound(id))
        }
    }

    pub fn change_state(&mut self, id: usize, state: TodoState) -> Result<(), TodoError> {
        let todo = self.todos.iter_mut().find(|todo| todo.id == id);

        if let Some(todo) = todo {
            todo.state = state;
            Ok(())
        } else {
            Err(TodoError::NotFound(id))
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TodoState {
    Open,
    InProgress,
    Done,
}

#[derive(Debug, PartialEq)]
pub struct Todo {
    id: usize,
    pub content: String,
    pub state: TodoState,
}

impl Todo {
    fn new(id: usize, content: &str) -> Self {
        Self {
            id,
            content: String::from(content),
            state: TodoState::Open,
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}

#[cfg(test)]
mod test_lib {
    use super::*;

    #[test]
    fn add_todo_adds_todo() {
        let mut manager = TodoManager::default();

        let content = "Lorem Ipsum";
        manager.add_todo(content);

        assert_eq!(manager.todos.len(), 1);
        assert_eq!(manager.todos[0].content, content);
        assert_eq!(manager.todos[0].state, TodoState::Open)
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

        assert_eq!(manager.todos[1].id, 2);
    }

    #[test]
    fn get_open_todos() {
        let mut manager = TodoManager::default();
        manager.add_todo("Lorem");
        manager.add_todo("Ipsum");
        manager.add_todo("Dolor");

        manager.todos[2].state = TodoState::InProgress;

        let open_todos = manager.get_by_state(TodoState::Open);

        assert_eq!(open_todos.len(), 2);

        assert_eq!(
            open_todos[0],
            &Todo {
                id: 1,
                content: String::from("Lorem"),
                state: TodoState::Open
            }
        );

        assert_eq!(
            open_todos[1],
            &Todo {
                id: 2,
                content: String::from("Ipsum"),
                state: TodoState::Open
            }
        );
    }

    #[test]
    fn get_in_progress_todos() {
        let mut manager = TodoManager::default();
        manager.add_todo("Lorem");
        manager.add_todo("Ipsum");
        manager.add_todo("Dolor");

        manager.todos[1].state = TodoState::InProgress;

        let in_progress = manager.get_by_state(TodoState::InProgress);

        assert_eq!(in_progress.len(), 1);

        assert_eq!(
            in_progress[0],
            &Todo {
                id: 2,
                content: String::from("Ipsum"),
                state: TodoState::InProgress
            }
        );
    }

    #[test]
    fn get_all_todos() {
        let mut manager = TodoManager::default();

        manager.add_todo("Lorem");
        manager.add_todo("Ipsum");
        manager.add_todo("Dolor");
        manager.add_todo("Sit");

        let todos = manager.get_all();

        assert_eq!(todos.len(), 4)
    }

    #[test]
    fn get_existing_todo_by_id() {
        let mut manager = TodoManager::default();

        manager.add_todo("This has id 1");
        manager.add_todo("This has id 2");
        manager.add_todo("This has id 3");

        let todo = manager.get_by_id(3);

        assert_eq!(
            todo,
            Some(&Todo {
                id: 3,
                content: String::from("This has id 3"),
                state: TodoState::Open
            })
        )
    }

    #[test]
    fn try_getting_non_existent_todo_by_id() {
        let manager = TodoManager::default();

        let todo = manager.get_by_id(42);

        assert_eq!(todo, None)
    }

    #[test]
    fn get_id() {
        let todo = Todo::new(42, "Lorem Ipsum");
        assert_eq!(todo.get_id(), 42);
    }

    #[test]
    fn edit_existing_todo_content_succeeds() {
        let mut manager = TodoManager::default();

        let new_id = manager.add_todo("This is a nice TODO.");

        let test_content = "This is even better!";
        let result = manager.edit_content(new_id, test_content);

        assert_eq!(result, Ok(()));

        let updated_content = &manager.get_by_id(new_id).unwrap().content;
        assert_eq!(updated_content, test_content)
    }

    #[test]
    fn edit_nonexistent_todo_content_fails() {
        let mut manager = TodoManager::default();

        let result = manager.edit_content(1, "Some content.");
        assert_eq!(result, Err(TodoError::NotFound(1)))
    }

    #[test]
    fn change_existing_todo_state_succeeds() {
        let mut manager = TodoManager::default();
        let new_id = manager.add_todo("Good to do.");

        let new_state = TodoState::InProgress;

        let result = manager.change_state(new_id, new_state);
        assert_eq!(result, Ok(()));

        let updated_state = &manager.get_by_id(new_id).unwrap().state;
        assert_eq!(*updated_state, new_state);
    }

    #[test]
    fn change_nonexistent_todo_state_fails() {
        let mut manager = TodoManager::default();

        let result = manager.change_state(42, TodoState::Done);
        assert_eq!(result, Err(TodoError::NotFound(42)));
    }
}
