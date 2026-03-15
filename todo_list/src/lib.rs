// unit testing

pub mod data;

#[cfg(test)]
mod tests {
    use crate::data::{AppState, Task};
    
    #[test]
    fn test_task_new() {
        let task = Task::new("Milk", "Groceries", 1);
        assert_eq!(task.title, "Milk");
        assert_eq!(task.completed, false);
        assert_eq!(task.index, 1);
    }

    #[test]
    fn test_app_state_next_index() {
        let mut state = AppState { tasks: vec![], next_index: 0 };
        let task1 = Task::new("A", "1", state.next_index);
        state.tasks.push(task1);
        state.next_index += 1;
        assert_eq!(state.tasks[0].index, 0);
        assert_eq!(state.next_index, 1);
    }
}
