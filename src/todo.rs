use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize, ValueEnum)]
pub enum TodoStatus {
    Open,
    InProgress,
    Done,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Todo {
    id: usize,
    pub content: String,
    pub status: TodoStatus,
}

impl Todo {
    pub fn new(id: usize, content: &str) -> Self {
        Self {
            id,
            content: String::from(content),
            status: TodoStatus::Open,
        }
    }

    pub fn get_id(&self) -> usize {
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
        assert_eq!(todo.get_id(), 42);
    }
}
