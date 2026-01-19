use chrono::{DateTime, Utc};
use serde::{Serialize,Deserialize};

// Data model
#[derive(Serialize,Deserialize)]
pub struct Task {
    description: String,
    completed: bool,
    deadline: Option<DateTime<Utc>>
}

pub enum TaskStatus {
    Completed,
    Pending,
    Overdue,
}

// NOTE: struct for parsed task representation for higher level rendering
pub struct ParsedTask {
    pub description: String,
    pub deadline: String,
    pub status: String,
}

impl Task {
    pub fn new(description: String, deadline: Option<DateTime<Utc>>) -> Task {
        return Task{
            description,
            completed: false,
            deadline,
        }
    }

    pub fn status(&self) -> TaskStatus {
        if self.completed {
            return TaskStatus::Completed;
        }
        match &self.deadline {
            None => TaskStatus::Pending,
            Some(d) => {
                let current_time = Utc::now();
                if d < &current_time {
                    TaskStatus::Overdue
                } else {
                    TaskStatus::Pending
                }
            }
        }
    }
    
    pub fn show(&self) {
        let parsed_deadline: String = match &self.deadline {
            None => "No deadline".to_string(),
            Some(d) => d.format("%Y-%m-%d %H:%M").to_string(),
        };

        let parsed_status: String = match &self.status() {
            TaskStatus::Completed => "Completed".to_string(),
            TaskStatus::Pending => "Pending".to_string(),
            TaskStatus::Overdue => "Overdue".to_string(),
        };

        println!("{} | {} | {}", self.description, parsed_deadline, parsed_status);
    }
    
    pub fn finish(&mut self) {
        self.completed = true;
    }
}

// Convert task model to parsed task view
impl From<Task> for ParsedTask {
    fn from(task: Task) -> ParsedTask {
        let parsed_deadline = match task.deadline {
            None => String::from("No deadline"),
            Some(d) => d.format("%Y-%m-%d %H:%M").to_string(),
        };

        let parsed_status = match task.status() {
            TaskStatus::Completed => String::from("Completed"),
            TaskStatus::Pending   => String::from("Pending"),
            TaskStatus::Overdue   => String::from("Overdue"),
        };

        ParsedTask {
            description: task.description,
            deadline: parsed_deadline,
            status: parsed_status
        }
    }
}
