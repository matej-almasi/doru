use clap::{Parser, Subcommand};
use rudo::TodoManager;

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

    let mut todo_manager = TodoManager::default();

    match cli.command {
        Commands::Add { content } => {
            todo_manager.add_todo(&content);
        }
    }

    // temporary: list all todos after command is executed
    for todo in todo_manager.get_all() {
        println!("{todo}");
    }
}
