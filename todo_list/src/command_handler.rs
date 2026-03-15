// "Subcommand" turns enum into CLI subcommands
use clap::{Subcommand};
use crate::data::{AppState, Task};

#[derive(Debug, Subcommand)]
pub enum Commands {
    List, // cargo run -- list
    Add { title: String, description: String }, // cargo run -- add ...
    Complete { index: usize }, // cargo run -- complete ...
    Delete { index: usize }, // cargo run -- delete ...
}

pub fn handle_command(state: &mut AppState, command: &Commands) {
    match command {
        Commands::Add { title, description } => {
            handle_add(state, title, description);
        }
        Commands::List => {
            handle_list(state);
        }
        Commands::Complete { index} => {
            handle_complete(state, *index);
        }
        Commands::Delete { index } => {
            handle_delete(state, *index);
        }
    }
}

// ---------------------- PRIVATE HELPERS ----------------------
fn handle_add(state: &mut AppState, title: &str, description: &str) {
    let task = Task::new(&title, &description, state.next_index);
    state.tasks.push(task);
    state.next_index += 1;
    println!("Added: {:?}", state.tasks.last().unwrap());
}

fn handle_list(state: &AppState) {
    if state.tasks.is_empty() {
        println!("No tasks found.");
    } else {
        for task in &state.tasks {
            println!("{:?}", task);
        }
    }
}

fn handle_complete(state: &mut AppState, index: usize) {
    let task = state.tasks.iter_mut().find(|t| t.index == index).map(|t| t.completed = true);

    if task.is_none() {
        println!("No task found with index: {}", index);
    } else {
        println!("Completed task with index: {}", index);
    }
}

fn handle_delete(state: &mut AppState, index: usize) {
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