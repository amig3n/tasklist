use chrono::DateTime;
use chrono::Utc;

struct Task {
    name: String,
    completed: bool,
    deadline: Option<DateTime<Utc>>
}

impl Task {
    fn new(name: String, deadline: Option<DateTime<Utc>>) -> Task {
        return Task{
            name: name,
            completed: false,
            deadline: deadline
        }
    }
    
    fn show(&self) {
        let parsedDeadline: String= match &self.deadline {
            None => "No deadline".to_string(),
            Some(d) => d.format("%Y-%m-%d %H:%M").to_string(),
        };
        let parsedStatus: String = match &self.completed {
            true => "Completed".to_string(),
            false => "Not completed".to_string(),
        };
        println!("{} | {} | {}", self.name, parsedDeadline, parsedStatus);
    }
    
    fn finish(&mut self) {
        self.completed = true;
    }
}

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
