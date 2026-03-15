pub mod data;
pub mod persistence;
pub mod command_handler;
pub mod input_handler;

// used for better error handling and outputs
use anyhow::{Result};
use crate::data::{AppState};
use crate::persistence::{load_state, save_state};
use crate::command_handler::handle_command;
use crate::input_handler::get_input;

fn main() -> Result<()> {
    let args = get_input();
    let mut state: AppState = load_state()?;
    handle_command(&mut state, &args.command);
    save_state(&state)?;
    Ok(())
}