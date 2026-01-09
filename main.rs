use chrono::Utc;

mod task;
use task::Task;

mod tasklist;
use tasklist::TaskList;

fn main() {

    let mut task_list = TaskList::new();

    let deadlinedTask = Task::new("Finish this program".to_string(), Some(Utc::now()));
    task_list.add(deadlinedTask);
    let notDeadlinedTask = Task::new("Test task without deadline".to_string(), None);
    task_list.add(notDeadlinedTask);

    task_list.show();
   


}
