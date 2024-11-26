use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use crate::todo::Todo;

use super::{TodoStorage, TodoStorageError};

pub struct JsonStorage {}

impl TodoStorage for JsonStorage {
    fn load(path: &Path) -> Result<Vec<Todo>, TodoStorageError> {
        let json = fs::read_to_string(path)
            .map_err(|_| TodoStorageError::FileError(path.to_path_buf()))?;

        if json.trim().is_empty() {
            return Ok(vec![]);
        }

        let todos: Vec<Todo> = serde_json::from_str(&json)
            .map_err(|_| TodoStorageError::ParseError(path.to_path_buf()))?;

        Ok(todos)
    }

    fn save(todos: &[&Todo], path: &Path) -> Result<(), TodoStorageError> {
        let json = serde_json::to_string(todos).map_err(|_| TodoStorageError::SerializeError)?;

        let mut file = File::options()
            .write(true)
            .truncate(true)
            .open(path)
            .map_err(|_| TodoStorageError::FileError(path.to_path_buf()))?;

        write!(file, "{json}").map_err(|_| TodoStorageError::FileError(path.to_path_buf()))?;

        Ok(())
    }
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
    fn read_empty_file_returns_empty_vector() {
        let test_file = NamedTempFile::new().unwrap();

        let parsed_todos = JsonStorage::load(test_file.path()).unwrap();

        assert_eq!(parsed_todos, vec![]);
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

    #[test]
    fn parse_invalid_json_fails() {
        let mut test_file = NamedTempFile::new().unwrap();
        writeln!(test_file, "Lorem ipsum not a todo list json").unwrap();

        let parsed_todos = JsonStorage::load(test_file.path());

        assert_eq!(
            parsed_todos,
            Err(TodoStorageError::ParseError(test_file.path().to_path_buf()))
        );
    }

    #[test]
    fn saving_to_existing_file_succeeds() {
        let test_file = NamedTempFile::new().unwrap();

        let todos = vec![
            Todo::new(0, "Lorem"),
            Todo::new(1, "Ipsum"),
            Todo::new(2, "Dolor"),
        ];

        let referenced_todos: Vec<&Todo> = todos.iter().collect();

        JsonStorage::save(&referenced_todos, test_file.path()).unwrap();

        assert_eq!(
            fs::read_to_string(test_file.path()).unwrap(),
            serde_json::to_string(&todos).unwrap()
        )
    }

    #[test]
    fn saving_to_nonexistent_file_fails() {
        let nonexistent_path = Path::new("nonexistent/path.json");

        let todos = [Todo::new(0, "Lorem")];

        let referenced_todos: Vec<&Todo> = todos.iter().collect();

        let result = JsonStorage::save(&referenced_todos, nonexistent_path);

        assert_eq!(
            result,
            Err(TodoStorageError::FileError(nonexistent_path.to_path_buf()))
        )
    }
}
