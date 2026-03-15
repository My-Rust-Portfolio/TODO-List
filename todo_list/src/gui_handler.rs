use crate::data::{AppState, Task};
use crate::persistence::save_state_file;
use eframe::egui::{self};

pub struct TodoApp {
    app_state: AppState,

    show_add_window: bool,
    draft_title: String,
    draft_desc: String,
}

impl TodoApp {
    pub fn new(state: AppState) -> Self {
        TodoApp {
            app_state: state,
            show_add_window: false,
            draft_title: String::new(),
            draft_desc: String::new(),
        }
    }
}

impl eframe::App for TodoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My Todo List");

            let add_task = ui.button("➕ Add Task");
            if add_task.clicked() {
                self.show_add_window = true;
            }

            ui.separator();

            let mut tasks_to_delete: Vec<usize> = vec![];

            for task in &mut self.app_state.tasks {
                ui.horizontal(|ui| {
                    let status = if task.completed { "✅" } else { "    " };
                    let complete_button = ui.button(status).on_hover_text("Complete task");
                    if complete_button.clicked() {
                        task.completed = !task.completed;
                    }
                    ui.label(format!("{}: {}", task.title, task.description));

                    let delete_button = ui.button("❌").on_hover_text("Delete task Permanently");
                    if delete_button.clicked() {
                        tasks_to_delete.push(task.index);
                    }
                });
            }

            for index in tasks_to_delete {
                self.app_state.tasks.retain(|t| t.index != index);
            }

            if self.show_add_window {
                egui::Window::new("Add New Task")
                    .collapsible(false)
                    .resizable(false)
                    .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::new(0.0, 0.0))
                    .show(ctx, |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Title:");
                            ui.text_edit_singleline(&mut self.draft_title);
                        });

                        ui.horizontal(|ui| {
                            ui.label("Description:");
                            ui.text_edit_singleline(&mut self.draft_desc);
                        });

                        ui.separator();

                        let create_task_button = ui.button("Create Task");
                        if create_task_button.clicked() {
                            let new_task = Task::new(
                                &self.draft_title,
                                &self.draft_desc,
                                self.app_state.next_index,
                            );
                            self.app_state.tasks.push(new_task);
                            self.app_state.next_index += 1;
                            self.draft_title.clear();
                            self.draft_desc.clear();
                            self.show_add_window = false;
                        }

                        let cancel_button = ui.button("Cancel");
                        if cancel_button.clicked() {
                            self.draft_title.clear();
                            self.draft_desc.clear();
                            self.show_add_window = false;
                        }
                    });
            }
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // Save the state when the application exits
        if let Err(e) = save_state_file(&self.app_state) {
            eprintln!("Failed to save state: {}", e);
        }
    }
}
