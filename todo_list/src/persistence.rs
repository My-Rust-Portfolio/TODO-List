// used for better error handling and outputs
use anyhow::{Context, Result};

// file handling
use std::{fs, path::PathBuf};

use crate::data::{AppState};

pub fn load_state() -> Result<AppState> {
    let path = state_file();
    if path.exists() {
        let json = fs::read_to_string(&path).context("Failed to read state.json")?;
        serde_json::from_str(&json).context("Failed to parse state.json")
    } else {
        Ok(AppState { tasks: vec![], next_index: 0 }) // Start with empty state
    }
}

pub fn save_state(state: &AppState) -> Result<()> {
    let json = serde_json::to_string_pretty(state).context("Failed to serialize state")?;
    fs::write(state_file(), json).context("Failed to write state.json")
}

// ---------------------- PRIVATE HELPERS ----------------------
fn state_file() -> PathBuf {
    PathBuf::from("state.json")
}
