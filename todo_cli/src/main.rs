use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufReader, Write};
use crossterm::{execute, terminal::{Clear, ClearType}};
use std::io::stdout;

const FILE_PATH: &str = "todo_list.json";

// Task structure
#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    completed: bool,
}

impl Task {
    fn new(description: String) -> Task {
        Task {
            description,
            completed: false,
        }
    }
}

// CLI commands
#[derive(Parser)]
#[command(name = "To-Do CLI")]
#[command(about = "A simple command-line to-do list manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { description: String },
    View,
    Remove { index: usize },
}

// Load tasks from the JSON file
fn load_tasks() -> io::Result<Vec<Task>> {
    let file = File::open(FILE_PATH);
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            let tasks: Vec<Task> = serde_json::from_reader(reader)?;
            Ok(tasks)
        }
        Err(_) => Ok(vec![]), // Return an empty list if the file doesn't exist
    }
}

// Save tasks to the JSON file
fn save_tasks(tasks: &Vec<Task>) -> io::Result<()> {
    let mut file = File::create(FILE_PATH)?; // Declare as mutable
    let json = serde_json::to_string_pretty(tasks)?;
    write!(file, "{}", json)?;
    Ok(())
}

// Clear the terminal for a clean view
fn clear_terminal() -> io::Result<()> {
    execute!(stdout(), Clear(ClearType::All))?;
    Ok(())
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    // Load the current list of tasks from the file
    let mut tasks = load_tasks()?;

    // Clear the terminal for a clean look
    clear_terminal()?;

    // Handle the different commands
    match &cli.command {
        Commands::Add { description } => {
            let task = Task::new(description.to_string());
            tasks.push(task);
            save_tasks(&tasks)?;
            println!("Task added successfully!");
        }
        Commands::View => {
            if tasks.is_empty() {
                println!("No tasks found.");
            } else {
                for (i, task) in tasks.iter().enumerate() {
                    let status = if task.completed { "[x]" } else { "[ ]" };
                    println!("{} {} - {}", i, status, task.description);
                }
            }
        }
        Commands::Remove { index } => {
            if *index < tasks.len() {
                tasks.remove(*index);
                save_tasks(&tasks)?;
                println!("Task removed successfully!");
            } else {
                println!("Invalid index.");
            }
        }
    }

    Ok(())
}
