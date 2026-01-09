use chrono::Utc;

mod task;
use task::Task;

mod tasklist;
use tasklist::TaskList;

fn main() {

    let mut task_list = TaskList::new();

    task_list.add(
        Task::new("Finish this program".to_string(), Some(Utc::now()))
    );
    task_list.add(
        Task::new("Test task without deadline".to_string(), None)
    );

    task_list.show();

    task_list.finish(0);
    task_list.finish(1);
    task_list.finish(2);

    task_list.add(
        Task::new("Add some test code".to_string(), None)
    );

    task_list.show();

   


}
