pub mod command_handler;
pub mod data;
pub mod gui_handler;
pub mod input_handler;
pub mod persistence;

// unit testing
#[cfg(test)]
mod tests {
    use crate::command_handler::{Commands, handle_command};
    use crate::data::{AppState, Task};
    use crate::persistence::{
        delete_state_file_name, load_state_from_file_name, save_state_to_file_name, state_file_name,
    };

    #[test]
    fn test_task_new() {
        let task = Task::new("Milk", "Groceries", 1);
        assert_eq!(task.title, "Milk");
        assert_eq!(task.completed, false);
        assert_eq!(task.index, 1);
    }

    #[test]
    fn test_app_state_save_load_delete_file() {
        let mut state = AppState {
            tasks: vec![],
            next_index: 0,
        };
        let task = Task::new("Milk", "Groceries", state.next_index);
        state.tasks.push(task);
        state.next_index += 1;
        let file_name = "test_state_1.json";
        save_state_to_file_name(&state, file_name).expect("Failed to save state");
        let loaded_state = load_state_from_file_name(file_name).expect("Failed to load state");
        assert_eq!(loaded_state.tasks.len(), 1);
        assert_eq!(loaded_state.tasks[0].title, "Milk");
        assert_eq!(loaded_state.tasks[0].description, "Groceries");
        assert_eq!(loaded_state.tasks[0].completed, false);
        assert_eq!(loaded_state.next_index, 1);
        delete_state_file_name(file_name).expect("Failed to delete state file");
        let path_to_file = state_file_name(file_name);
        assert!(!path_to_file.exists(), "State file was not deleted");
    }

    #[test]
    fn test_app_state_add_command() {
        let mut state = AppState {
            tasks: vec![],
            next_index: 0,
        };
        handle_command(
            &mut state,
            &Commands::Add {
                title: "Task 1".to_string(),
                description: "Description 1".to_string(),
            },
        );
        assert_eq!(state.tasks.len(), 1);
        assert_eq!(state.tasks[0].title, "Task 1");
        assert_eq!(state.tasks[0].description, "Description 1");
    }

    #[test]
    fn test_app_state_complete_command() {
        let mut state = AppState {
            tasks: vec![],
            next_index: 0,
        };
        handle_command(
            &mut state,
            &Commands::Add {
                title: "Task 1".to_string(),
                description: "Description 1".to_string(),
            },
        );
        handle_command(&mut state, &Commands::Complete { index: 0 });
        assert_eq!(state.tasks[0].completed, true);
    }

    #[test]
    fn test_app_state_delete_command() {
        let mut state = AppState {
            tasks: vec![],
            next_index: 0,
        };
        handle_command(
            &mut state,
            &Commands::Add {
                title: "Task 1".to_string(),
                description: "Description 1".to_string(),
            },
        );
        handle_command(&mut state, &Commands::Delete { index: 0 });
        assert_eq!(state.tasks.len(), 0);
    }

    #[test]
    fn test_app_state_next_id() {
        let mut state = AppState {
            tasks: vec![],
            next_index: 0,
        };
        handle_command(
            &mut state,
            &Commands::Add {
                title: "Task 0".to_string(),
                description: "Description 0".to_string(),
            },
        );
        let file_name = "test_state_2.json";
        save_state_to_file_name(&state, file_name).expect("Failed to save state");

        state = load_state_from_file_name(file_name).expect("Failed to load state");
        handle_command(
            &mut state,
            &Commands::Add {
                title: "Task 1".to_string(),
                description: "Description 1".to_string(),
            },
        );
        assert_eq!(state.tasks[1].index, 1);
        save_state_to_file_name(&state, file_name).expect("Failed to save state");

        state = load_state_from_file_name(file_name).expect("Failed to load state");
        handle_command(
            &mut state,
            &Commands::Add {
                title: "Task 2".to_string(),
                description: "Description 2".to_string(),
            },
        );
        assert_eq!(state.tasks[2].index, 2);
        save_state_to_file_name(&state, file_name).expect("Failed to save state");

        state = load_state_from_file_name(file_name).expect("Failed to load state");
        handle_command(&mut state, &Commands::Delete { index: 1 });
        assert_eq!(state.tasks[1].index, 2);
        save_state_to_file_name(&state, file_name).expect("Failed to save state");

        state = load_state_from_file_name(file_name).expect("Failed to load state");
        handle_command(
            &mut state,
            &Commands::Add {
                title: "Task 3".to_string(),
                description: "Description 3".to_string(),
            },
        );
        assert_eq!(state.tasks[2].index, 3);
        save_state_to_file_name(&state, file_name).expect("Failed to save state");

        delete_state_file_name(file_name).expect("Failed to delete state file");
    }
}
