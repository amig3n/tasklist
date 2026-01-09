use crate::task::Task;

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

        while task_index < self.tasks.len() {
            let current_task = &self.tasks[task_index];
            print!("{} | ", task_index);
            current_task.show();
            task_index += 1;
        }
    }
}
