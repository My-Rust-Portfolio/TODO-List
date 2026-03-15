use anyhow::Result;
// The GUI can use the exact same data and persistence logic!
use todo_list::data::AppState;
use todo_list::persistence::{load_state_file};

fn main() -> Result<()> {
    let state:AppState = load_state_file()?;
    println!("GUI loaded {} tasks!", state.tasks.len());
    
    // TODO implement GUI
    
    Ok(())
}