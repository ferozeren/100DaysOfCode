use clap::{Parser, Subcommand, ValueEnum};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    name: String,
    priority: Priority,
    completed: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum, Serialize, Deserialize)]
enum Priority {
    Low,
    Normal,
    High,
}

#[derive(Parser, Debug)]
#[command(version, about = "A minimal task manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a new task
    Add {
        name: String,
        #[arg(short, long, value_enum, default_value_t = Priority::Normal)]
        priority: Priority,
    },
    /// List all pending tasks
    List,
    /// Mark a task as complete
    Complete { id: u32 },
}

fn get_path() -> PathBuf {
    ProjectDirs::from("com", "User", "TaskCLI")
        .map(|proj| {
            let _ = fs::create_dir_all(proj.config_dir());
            proj.config_dir().join("tasks.json")
        })
        .unwrap_or_else(|| PathBuf::from("tasks.json"))
}

fn load(path: &PathBuf) -> Vec<Task> {
    fs::read_to_string(path)
        .ok()
        .and_then(|c| serde_json::from_str(&c).ok())
        .unwrap_or_default()
}

fn save(path: &PathBuf, tasks: &[Task]) {
    if let Ok(json) = serde_json::to_string_pretty(tasks) {
        let _ = fs::write(path, json);
    }
}

fn main() {
    let cli = Cli::parse();
    let path = get_path();
    let mut tasks = load(&path);

    match cli.command {
        Commands::Add { name, priority } => {
            let id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
            tasks.push(Task {
                id,
                name,
                priority,
                completed: false,
            });
            save(&path, &tasks);
            println!("Added task #{}", id);
        }
        Commands::List => {
            let pending: Vec<_> = tasks.iter().filter(|t| !t.completed).collect();
            if pending.is_empty() {
                println!("No tasks.");
            } else {
                for t in pending {
                    println!("[ ] {} - {} ({:?})", t.id, t.name, t.priority);
                }
            }
        }
        Commands::Complete { id } => {
            if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
                task.completed = true;
                save(&path, &tasks);
                println!("Completed task #{}", id);
            } else {
                println!("Task not found.");
            }
        }
    }
}
