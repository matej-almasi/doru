#[derive(Default)]
pub struct TodoManager {
    id_counter: usize,
    todos: Vec<Todo>,
}

impl TodoManager {
    pub fn add_todo(&mut self, content: &str) {
        self.id_counter += 1;
        self.todos.push(Todo::new(self.id_counter, content));
    }

    pub fn get_by_state(&self, state: TodoState) -> Vec<&Todo> {
        self.todos
            .iter()
            .filter(|todo| todo.state == state)
            .collect()
    }

    pub fn get_all(&self) -> Vec<&Todo> {
        self.todos.iter().collect()
    }
}

#[derive(PartialEq, Debug)]
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
    fn add_todo() {
        let mut manager = TodoManager::default();

        let content = "Lorem Ipsum";
        manager.add_todo(content);

        assert_eq!(manager.todos.len(), 1);
        assert_eq!(manager.todos[0].content, content);
        assert_eq!(manager.todos[0].state, TodoState::Open)
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
    fn get_id() {
        let todo = Todo::new(42, "Lorem Ipsum");
        assert_eq!(todo.get_id(), 42);
    }
}
