// autogenerate JSON
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    title: String,
    description: String,
    pub completed: bool,
    pub index: usize,
}

impl Task  {
    pub fn new(title: &str, description: &str, index: usize) -> Self {
        Task {
            title: title.to_string(),
            description: description.to_string(),
            completed: false,
            index,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppState {
    pub tasks: Vec<Task>,
    pub next_index: usize,
}