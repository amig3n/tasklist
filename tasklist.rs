use crate::task::Task;
use std::fs;
use std::path::Path;

pub struct TaskList {
    tasks: Vec<Task>
}

impl TaskList {
    // init new tasklist
    pub fn new() -> TaskList {
        return TaskList{
            tasks: Vec::new(),
        }
    }

    pub fn save(&mut self, path: &Path) -> Result<(), &str> {
        let parsed_json = match serde_json::to_string_pretty(&self.tasks) {
            Ok(json) => json,
            Err(e) => {
                return Err("Unable to serialize tasklist");
            }
        };

        match fs::write(path, parsed_json) {
            Ok(_) => return Ok(()),
            Err(e) => {
                return Err("Writing tasklist file failed");
            }
        }
    }
            
    pub fn load(path: &Path) -> Result<TaskList, &str> {
        let raw_content;

        match fs::read_to_string(path) {
            Ok(data) => {
                // capture the data
                raw_content = data;
            }

            Err(_) => {
                return Err("Unable to open the tasklist file");
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
                return Err("Unable to deserialize task list");
            }
        }

        return Ok(
            TaskList{
                tasks: tasks,
            }
        );
    }


    pub fn add(&mut self, task: Task){
        self.tasks.push(task);
    }

    pub fn finish(&mut self, task_index: usize) -> Result<(), &str> {
        
        match self.tasks.get_mut(task_index) {
             Some(task) => {
                 task.finish();
                 return Ok(());
             }

             None => { 
                 return Err("Task with received index not found");
             }
        };
    }

    /// Get mutable reference for single task
    pub fn get_single_task_mut(&mut self, task_index: usize) -> Result<&mut Task, &str> {
        // check if given task exists inside tasklist
        if task_index > 0 && task_index < self.tasks.len() {
            // return reference to the task
            return Ok(&mut self.tasks[task_index]);
        } else {
            return Err("Task does not exists");
        }
    }

    /// Get non-mutable reference to a single task
    pub fn get_single_task(&self, task_index: usize) -> Result<&Task, &str> {
        // check if given task exists inside tasklist
        if task_index > 0 && task_index < self.tasks.len() {
            // return reference to the task
            return Ok(&self.tasks[task_index]);
        } else {
            return Err("Task does not exists");
        }
    }

    // TODO return structured data for proper rendering
    // TODO rewrite this func to use get_single_task inside for loop
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

    pub fn delete(&mut self, task_index: usize) -> Result<(), &str> {
        if task_index > 0 && task_index < self.tasks.len() {
            self.tasks.remove(task_index);
            return Ok(());
        } else {
            return Err("Invalid task index");
        }
    }

}

