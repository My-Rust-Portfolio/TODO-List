// used for better error handling and outputs
use anyhow::{Result};
use todo_list::data::{AppState};
use todo_list::persistence::{load_state_file, save_state_file};
use todo_list::command_handler::handle_command;
use todo_list::input_handler::get_input;

fn main() -> Result<()> {
    let args = get_input();
    let mut state: AppState = load_state_file()?;
    handle_command(&mut state, &args.command);
    save_state_file(&state)?;
    Ok(())
}