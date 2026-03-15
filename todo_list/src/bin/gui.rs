use anyhow::Result;
use eframe::egui;
// The GUI can use the exact same data and persistence logic!
use todo_list::data::AppState;
use todo_list::persistence::load_state_file;

fn main() -> Result<(), eframe::Error> {
    let state: AppState = load_state_file().unwrap_or(AppState { tasks: vec![], next_index: 0 });
    
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

struct TodoApp {
    app_state: AppState,
}

impl TodoApp {
    fn new(state: AppState) -> Self {
        TodoApp { app_state: state }
    }
}

impl eframe::App for TodoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My Todo List");

            for task in &self.app_state.tasks {
                ui.horizontal(|ui| {
                    let status = if task.completed { "X" } else { "   " };
                    ui.label(format!("[{}] {}: {}", status, task.title, task.description));
                });
            }
        });
    }
}
