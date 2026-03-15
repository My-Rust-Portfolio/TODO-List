// used for better error handling and outputs
use anyhow::{Context, Result};

// file handling
use std::{fs, path::PathBuf};

use crate::data::{AppState};

pub fn load_state_from_file_name(file_name: &str) -> Result<AppState> {
    let path = state_file_name(file_name);
    if path.exists() {
        let json = fs::read_to_string(&path).context(format!("Failed to read {}", file_name))?;
        serde_json::from_str(&json).context(format!("Failed to parse {}", file_name))
    } else {
        Ok(AppState { tasks: vec![], next_index: 0 }) // Start with empty state
    }
}

pub fn save_state_to_file_name(state: &AppState, file_name: &str) -> Result<()> {
    let json = serde_json::to_string_pretty(state).context("Failed to serialize state")?;
    fs::write(state_file_name(file_name), json).context(format!("Failed to write {}", file_name))
}

pub fn delete_state_file_name(file_name: &str) -> Result<()> {
    let path = state_file_name(file_name);
    if path.exists() {
        fs::remove_file(path).context("Failed to delete state file")
    } else {
        Ok(())
    }
}

pub fn load_state_file() -> Result<AppState> {
    load_state_from_file_name("state.json")
}

pub fn save_state_file(state: &AppState) -> Result<()> {
    save_state_to_file_name(state, "state.json")
}

pub fn delete_state_file() -> Result<()> {
    delete_state_file_name("state.json")
}

pub fn state_file_name(file_name: &str) -> PathBuf {
    PathBuf::from(file_name)
}
