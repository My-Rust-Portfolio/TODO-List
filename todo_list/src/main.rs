
// "Parser" autogerentes args from struct
// "Subcommand" turns enum into CLI subcommands
use clap::{Parser, Subcommand};

// used for better error handling and outputs
use anyhow::{Context, Result};

// autogenerate JSON
use serde::{Deserialize, Serialize};

// file handling
use std::{fs, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    title: String,
    description: String,
    completed: bool,
    index: usize,
}

#[derive(Debug, Subcommand)]
enum Commands {
    List, // cargo run -- list
    Add { title: String, description: String }, // cargo run -- add ...
    Complete { index: usize }, // cargo run -- complete ...
}

#[derive(Debug, Parser)]
struct Args {
    #[command(subcommand)] // Tells clap to parse the subcommand into this field
    command: Commands,
}

fn main() -> Result<()> {
    let args = Args::parse();  // Fills from CLI args
    println!("Parsed: {:?}", args.command);

    let mut tasks: Vec<Task> = load_tasks()?; // Stub for now

    match args.command {
        Commands::Add { title, description } => {
            let task = Task { title, description, completed: false, index: tasks.len() };
            tasks.push(task);
            println!("Added: {:?}", tasks.last().unwrap());
        }
        Commands::List => {
            for task in &tasks {
                println!("{:?}", task);
            }
        }
        Commands::Complete { index} => {
            if index >= tasks.len() {
                eprintln!("Error: No task with index {}", index);
                return Ok(()); // Exit gracefully
            }

            tasks.iter_mut().find(|t| t.index == index).map(|t| t.completed = true);
            println!("Completed task with index: {}", index);
        }
    }

    save_tasks(&tasks)?;
    Ok(())
}

fn tasks_file() -> PathBuf {
    PathBuf::from("tasks.json")
}

fn load_tasks() -> Result<Vec<Task>> {
    let path = tasks_file();
    if path.exists() {
        let json = fs::read_to_string(&path).context("Failed to read tasks.json")?;
        serde_json::from_str(&json).context("Failed to parse tasks.json")
    } else {
        Ok(vec![])
    }
}

fn save_tasks(tasks: &Vec<Task>) -> Result<()> {
    let json = serde_json::to_string_pretty(tasks).context("Failed to serialize tasks")?;
    fs::write(tasks_file(), json).context("Failed to write tasks.json")
}