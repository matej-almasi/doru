//! Entry point for the `doru` application, serving as an example use of the
//! doru library. It defines the command-line interface (CLI) and handles the
//! execution of commands.

use std::{env, error::Error, fs, path::Path, path::PathBuf};

use clap::{Parser, Subcommand};
use doru::{
    storage::{self, TodoStorage},
    todo::TodoStatus,
    todo_manager::TodoManager,
};

/// CLI structure for the `doru` application.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The command to execute.
    #[command(subcommand)]
    command: Commands,

    /// Optional path to the todos file.
    #[arg(short, long, global = true)]
    path: Option<String>,
}

/// Available commands for the `doru` application.
#[derive(Subcommand)]
enum Commands {
    /// Add a new TODO item.
    Add { content: String },

    /// Edit the content of an existing TODO item.
    Edit { id: usize, content: String },

    /// List TODO items, optionally filtered by status.
    List { status: Option<TodoStatus> },

    /// Change the status of an existing TODO item.
    Status { id: usize, status: TodoStatus },

    /// Delete an existing TODO item.
    Delete { id: usize },
}

/// Main entry point for the `doru` application.
fn ain() {
    let cli = Cli::parse();

    let path = match cli.path {
        Some(value) => PathBuf::from(value),
        None => get_todos_path().unwrap_or_else(|e| panic!("{e}")),
    };

    ensure_storage_exists(&path).expect("Failed reaching storage path.");

    let todos = storage::JsonStorage::load(&path).unwrap_or_else(|e| panic!("{e}"));

    let mut todo_manager = TodoManager::new(todos);

    match cli.command {
        Commands::Add { content } => {
            todo_manager.add_todo(&content);
        }

        Commands::Edit { id, content } => todo_manager
            .edit_todo_content(id, &content)
            .unwrap_or_else(|e| println!("{e}")),

        Commands::List { status } => {
            let todos = if let Some(value) = status {
                todo_manager.todos_by_status(value)
            } else {
                todo_manager.all_todos()
            };

            for todo in todos {
                println!("{todo}");
            }
        }

        Commands::Status { id, status } => todo_manager
            .change_todo_status(id, status)
            .unwrap_or_else(|e| println!("{e}")),

        Commands::Delete { id } => todo_manager
            .delete_todo(id)
            .unwrap_or_else(|e| println!("{e}")),
    }

    storage::JsonStorage::save(&todo_manager.all_todos(), &path).unwrap_or_else(|e| panic!("{e}"));
}

/// Get the path to the todos file.
///
/// This function checks the `DORU_PATH` environment variable. If not set, it
/// defaults to `~/.doru/todos.json`.
///
/// # Errors
///
/// Returns an error if the user's home directory cannot be determined.
fn get_todos_path() -> Result<PathBuf, Box<dyn Error>> {
    if let Ok(env_path) = env::var("DORU_PATH") {
        Ok(PathBuf::from(env_path))
    } else if let Some(home_dir) = dirs::home_dir() {
        Ok(home_dir.join(".doru").join("todos.json"))
    } else {
        Err("Unable to determine home directory".into())
    }
}

/// Ensure that the storage file exists.
///
/// This function creates the necessary directories and file if they do not
/// exist.
///
/// # Errors
///
/// Returns an error if the directories or file cannot be created.
fn ensure_storage_exists(path: &Path) -> std::io::Result<()> {
    if !path.exists() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::File::create(path)?;
    }

    Ok(())
}
