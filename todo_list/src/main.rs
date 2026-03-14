// "Parser" autogerentes args from struct
// "Subcommand" turns enum into CLI subcommands
use clap::{Parser, Subcommand};
// used for better error handling and outputs
use anyhow::{Result};

pub mod data;
use crate::data::{AppState, Task};

pub mod persistence;
use crate::persistence::{load_state, save_state};

#[derive(Debug, Subcommand)]
enum Commands {
    List, // cargo run -- list
    Add { title: String, description: String }, // cargo run -- add ...
    Complete { index: usize }, // cargo run -- complete ...
    Delete { index: usize }, // cargo run -- delete ...
}

#[derive(Debug, Parser)]
struct Args {
    #[command(subcommand)] // Tells clap to parse the subcommand into this field
    command: Commands,
}

fn main() -> Result<()> {
    let args = Args::parse();  // Fills from CLI args, has some error handling built in
    println!("Parsed: {:?}", args.command);

    let mut state: AppState = load_state()?;

    match args.command {
        Commands::Add { title, description } => {
            let task = Task::new(&title, &description, state.next_index);
            state.tasks.push(task);
            state.next_index += 1;
            println!("Added: {:?}", state.tasks.last().unwrap());
        }
        Commands::List => {
            if state.tasks.is_empty() {
                println!("No tasks found.");
            } else {
                for task in &state.tasks {
                    println!("{:?}", task);
                }
            }
        }
        Commands::Complete { index} => {
            let task = state.tasks.iter_mut().find(|t| t.index == index).map(|t| t.completed = true);

            if task.is_none() {
                println!("No task found with index: {}", index);
            } else {
                println!("Completed task with index: {}", index);
            }
        }
        Commands::Delete { index } => {
            let task = state.tasks.iter().position(|t| t.index == index).map(|pos| state.tasks.swap_remove(pos));

            if task.is_none() {
                println!("No task found with index: {}", index);
            } else {
                println!("Deleted task with index: {}", index);
                if state.tasks.is_empty() {
                    state.next_index = 0; // Reset index if no tasks remain
                }
            }
        }
    }

    save_state(state)?;
    Ok(())
}