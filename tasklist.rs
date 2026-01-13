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
                return Err("Not able to serialize the json");
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
                return Err("Unable to open the tasklist file: {}");
            }
        }

        // container for tasks
        let tasks: Vec<Task>;
        match serde_json::from_str(&raw_content) {
            Ok(parsed_content) => {
                // serialization went ok -> move parsed content to tasks
                tasks = parsed_content;
            }

            Err(_) => {
                return Err("Unable to serialize task list");
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

    // TODO: rewrite this function to use Result<>
    pub fn finish(&mut self, task_index: usize) {
        
        match self.tasks.get_mut(task_index) {
             Some(task) => {
                 task.finish();
             }

             None => { 
                 println!("No task with index {} found", task_index+1);
             }
        };
    }

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

    pub fn delete(&mut self, task_index: usize) {
        if task_index > 0 && task_index < self.tasks.len() {
            self.tasks.remove(task_index);
            println!("Task no {} deleted", task_index);
        } else {
            println!("Task no {} does not exists", task_index);
        }
    }



}
