use anyhow::Result;
use eframe::egui::{self};
use todo_list::data::AppState;
use todo_list::gui_handler::TodoApp;
use todo_list::persistence::load_state_file;

fn main() -> Result<(), eframe::Error> {
    let state: AppState = load_state_file().unwrap_or(AppState {
        tasks: vec![],
        next_index: 0,
    });

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 500.0]) // Width, Height
            .with_title("Todo App"),
        ..Default::default()
    };

    eframe::run_native(
        "Todo App",
        options,
        Box::new(|_cc| Ok(Box::new(TodoApp::new(state)))),
    )
}
