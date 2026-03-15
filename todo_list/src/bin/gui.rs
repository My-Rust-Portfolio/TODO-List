use eframe::egui;
use anyhow::Result;
// The GUI can use the exact same data and persistence logic!
use todo_list::data::AppState;
use todo_list::persistence::{load_state_file};

fn main() -> Result<(), eframe::Error> {
    //let state:AppState = load_state_file()?;
    //println!("GUI loaded {} tasks!", state.tasks.len());
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 500.0]) // Width, Height
            .with_title("Todo App"),
        ..Default::default()
    };

    eframe::run_native(
        "Todo App",
        options,
        // Box::new creates a heap allocation (like std::make_unique)
        Box::new(|_cc| Ok(Box::new(TodoApp {}))),
    )
}

struct TodoApp {
}

impl eframe::App for TodoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        // CentralPanel takes up the whole window
        egui::CentralPanel::default().show(ctx, |ui| {
            
            // Draw a big header
            ui.heading("Hello World!");
            
            // Draw a normal label
            ui.label("Hello World again!"); 
        });
    }
}