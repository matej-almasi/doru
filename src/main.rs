use std::path::Path;

use clap::{Parser, Subcommand};
use rudo::{storage, storage::TodoStorage, TodoManager};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { content: String },
    // Edit { id: usize, content: String },
    // List,
    // Status { id: usize, status: String },
    // Delete { id: usize },
}

fn main() {
    let cli = Cli::parse();

    let path = Path::new("todos.json");

    let todos = storage::JsonStorage::load(path).unwrap_or_else(|e| panic!("{e}"));

    let mut todo_manager = TodoManager::new(todos);

    match cli.command {
        Commands::Add { content } => {
            todo_manager.add_todo(&content);
        }
    }

    // temporary: list all todos after command is executed
    for todo in todo_manager.get_all() {
        println!("{todo}");
    }

    storage::JsonStorage::save(&todo_manager.get_all(), path)
        .expect("Something went wrong saving the todos.");
}
