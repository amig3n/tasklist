use crate::task::Task;
use std::fs;
use std::path::Path;
use std::fmt;

pub struct TaskList {
    tasks: Vec<Task>
}

#[derive(Debug)]
pub enum TaskListError {
    SaveError,
    LoadError,
    SerializationError,
    DeserializationError,
    TaskNotFound,
    TaskInvalidIndex,
    GeneralError(String),
}

impl fmt::Display for TaskListError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskListError::SaveError => write!(f, "Error saving tasklist"),
            TaskListError::LoadError => write!(f, "Error loading tasklist"),
            TaskListError::SerializationError => write!(f, "Error serializing tasklist"),
            TaskListError::DeserializationError => write!(f, "Error deserializing tasklist"),
            TaskListError::TaskNotFound => write!(f, "Task not found"),
            TaskListError::TaskInvalidIndex => write!(f, "Invalid task index"),
            TaskListError::GeneralError(msg) => write!(f, "TaskList general error: {}", msg),
        }
    }
}

impl std::error::Error for TaskListError {}

impl From<TaskListError> for String {
    fn from(err: TaskListError) -> Self {
        err.to_string()
    }
}

impl TaskList {
    /// Create new empty tasklist
    pub fn new() -> TaskList {
        return TaskList{
            tasks: Vec::new(),
        }
    }

    /// Save tasklist
    pub fn save(&mut self, path: &Path) -> Result<(), TaskListError> {
        let parsed_json = match serde_json::to_string_pretty(&self.tasks) {
            Ok(json) => json,
            Err(e) => {
                return Err(TaskListError::SerializationError);
            }
        };

        match fs::write(path, parsed_json) {
            Ok(_) => return Ok(()),
            Err(e) => {
                return Err(TaskListError::SaveError);
            }
        }
    }
            
    /// Load tasklist
    pub fn load(path: &Path) -> Result<TaskList, TaskListError> {
        let raw_content;

        match fs::read_to_string(path) {
            Ok(data) => {
                // capture the data
                raw_content = data;
            }

            Err(_) => {
                return Err(TaskListError::LoadError);
            }
        }

        // container for tasks
        let tasks: Vec<Task>;
        match serde_json::from_str(&raw_content) {
            Ok(parsed_content) => {
                // deserialization went ok -> move parsed content to tasks
                tasks = parsed_content;
            }

            Err(_) => {
                return Err(TaskListError::DeserializationError);
            }
        }

        return Ok(
            TaskList{
                tasks: tasks,
            }
        );
    }

    /// Add new task to tasklist
    pub fn add(&mut self, task: Task){
        self.tasks.push(task);
    }

    /// Finish single task
    pub fn finish(&mut self, task_index: usize) -> Result<(), TaskListError> {
        match self.tasks.get_mut(task_index) {
             Some(task) => {
                 task.finish();
                 return Ok(());
             }

             None => { 
                 return Err(TaskListError::TaskNotFound);
             }
        };
    }

    /// Get mutable reference for single task
    pub fn get_single_task_mut(&mut self, task_index: usize) -> Result<&mut Task, TaskListError> {
        // check if given task exists inside tasklist
        if task_index > 0 && task_index < self.tasks.len() {
            // return reference to the task
            return Ok(&mut self.tasks[task_index]);
        } else {
            return Err(TaskListError::TaskInvalidIndex);
        }
    }

    /// Get non-mutable reference to a single task
    pub fn get_single_task(&self, task_index: usize) -> Result<&Task, TaskListError> {
        // check if given task exists inside tasklist
        if task_index > 0 && task_index < self.tasks.len() {
            // return reference to the task
            return Ok(&self.tasks[task_index]);
        } else {
            return Err(TaskListError::TaskInvalidIndex);
        }
    }

    // TODO return structured data for proper rendering
    // TODO rewrite this func to use get_single_task inside for loop
    /// List all tasks inside tasklist
    pub fn show(&self) {
        let mut task_index = 0;
        
        let tasks_list_length = self.tasks.len();
        if tasks_list_length == 0 {
            println!("No tasks to display");
        }

        while task_index < tasks_list_length {
            let current_task = &self.tasks[task_index];
            print!("{} | ", task_index);
            current_task.show();
            task_index += 1;
        }
    }

    /// Delete task from tasklist
    pub fn delete(&mut self, task_index: usize) -> Result<(), TaskListError> {
        if task_index > 0 && task_index < self.tasks.len() {
            self.tasks.remove(task_index);
            return Ok(());
        } else {
            return Err(TaskListError::TaskInvalidIndex);
        }
    }

}

