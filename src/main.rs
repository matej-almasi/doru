use std::{env, error::Error, fs, path::Path, path::PathBuf};

use clap::{Parser, Subcommand};
use rudo::{
    storage::{self, TodoStorage},
    todo::TodoStatus,
    TodoManager,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, global = true)]
    path: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    Add { content: String },
    Edit { id: usize, content: String },
    List,
    Status { id: usize, status: TodoStatus },
    Delete { id: usize },
}

fn main() {
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
            .edit_content(id, &content)
            .unwrap_or_else(|e| println!("{e}")),

        Commands::List => {
            for todo in todo_manager.get_all() {
                println!("{todo}");
            }
        }

        Commands::Status { id, status } => todo_manager
            .change_status(id, status)
            .unwrap_or_else(|e| println!("{e}")),

        Commands::Delete { id } => todo_manager
            .delete_todo(id)
            .unwrap_or_else(|e| println!("{e}")),
    }

    storage::JsonStorage::save(&todo_manager.get_all(), &path).unwrap_or_else(|e| panic!("{e}"));
}

fn get_todos_path() -> Result<PathBuf, Box<dyn Error>> {
    if let Ok(env_path) = env::var("RUDO_PATH") {
        Ok(PathBuf::from(env_path))
    } else if let Some(home_dir) = dirs::home_dir() {
        Ok(home_dir.join(".rudo").join("todos.json"))
    } else {
        Err("Unable to determine home directory".into())
    }
}

fn ensure_storage_exists(path: &Path) -> std::io::Result<()> {
    if !path.exists() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::File::create(path)?;
    }

    Ok(())
}
