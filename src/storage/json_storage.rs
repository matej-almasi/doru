use std::{fs, path::Path};

use crate::Todo;

use super::{TodoStorage, TodoStorageError};

pub struct JsonStorage {}

impl TodoStorage for JsonStorage {
    fn load(path: &Path) -> Result<Vec<Todo>, TodoStorageError> {
        let json = fs::read_to_string(path)
            .map_err(|_| TodoStorageError::FileError(path.to_path_buf()))?;

        let todos: Vec<Todo> = serde_json::from_str(&json)
            .map_err(|_| TodoStorageError::ParseError(path.to_path_buf()))?;

        Ok(todos)
    }
    // fn save(todos: &[crate::Todo], path: &Path) -> Result<(), crate::TodoStorageError> {}
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;
    use std::io::Write;
    use tempfile::{self, NamedTempFile};

    #[test]
    fn read_and_parse_valid_file_succeeds() {
        let todos = vec![
            Todo::new(0, "Lorem"),
            Todo::new(1, "Ipsum"),
            Todo::new(2, "Dolor"),
        ];

        let todos_json = json!(todos).to_string();

        let mut test_file = NamedTempFile::new().unwrap();
        writeln!(test_file, "{todos_json}").unwrap();

        let parsed_todos = JsonStorage::load(test_file.path()).unwrap();

        assert_eq!(parsed_todos, todos);
    }

    #[test]
    fn read_nonexistent_file_fails() {
        let nonexistent_path = Path::new("/nonexistent/path.json");
        let result = JsonStorage::load(nonexistent_path);
        assert_eq!(
            result,
            Err(TodoStorageError::FileError(nonexistent_path.to_path_buf()))
        )
    }
}
