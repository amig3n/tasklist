use chrono::Utc;

mod task;
use task::Task;

mod tasklist;
use tasklist::TaskList;

fn main() {

    let path: &str = "/home/andrzej/tasklist.json";
    let mut task_list = TaskList::load(&path);

    task_list.show();


    //task_list.save();




   


}
