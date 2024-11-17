use std::fmt::Display;

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
    pub fn new(id: usize, content: &str) -> Self {
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

impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tick = if self.state == TodoState::Done {
            "x"
        } else {
            " "
        };

        write!(
            f,
            "[{}] {:<20} [{:<12?}] (ID: {})",
            tick, self.content, self.state, self.id
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_id() {
        let todo = Todo::new(42, "Lorem Ipsum");
        assert_eq!(todo.get_id(), 42);
    }
}
