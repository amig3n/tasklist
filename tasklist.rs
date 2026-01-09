use crate::task::Task;
use std::fs;

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

    pub fn save(&mut self, path: &str) {
        let parsed_json = serde_json::to_string_pretty(&self.tasks)
            .expect("Failed to serialize tasks data");

        fs::write(path, parsed_json)
            .expect("Failed to save tasks file");
    }
            
    pub fn load(path: &str) -> TaskList {
        let raw_content;

        match fs::read_to_string(path) {
            Ok(data) => {
                // capture the data
                raw_content = data;
            }

            Err(_) => {
                println!("Cannot load file tasklist file: {}", path);
                // create blank tasklist if load failed
                return TaskList::new();
            }
        }

        // container for tasks
        let tasks: Vec<Task>;

        match serde_json::from_str(&raw_content) {
            Ok(parsed_content) => {
                // serialization went ok -> move parsed content to tasks
                tasks = parsed_content;
            }

            Err(e) => {
                println!("Unable to serialize task list: {}", e);
                return TaskList::new();
            }
        }

        return TaskList{
            tasks: tasks,
        }
    }


    pub fn add(&mut self, task: Task){
        self.tasks.push(task);
    }

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
