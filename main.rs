use chrono::Utc;

mod task;
use task::Task;

fn main() {

let mut deadlinedTask = Task::new("Finish this program".to_string(), Some(Utc::now()));

deadlinedTask.show();

let mut notDeadlinedTask = Task::new("Test task without deadline".to_string(), None);
notDeadlinedTask.show();

deadlinedTask.finish();
notDeadlinedTask.finish();

deadlinedTask.show();
notDeadlinedTask.show();
}
