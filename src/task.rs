use chrono::{DateTime, Utc};
use serde::{Serialize,Deserialize};
use colored::*;

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
    
    pub fn finish(&mut self) {
        self.completed = true;
    }
}

// Convert task model to parsed task view
impl From<Task> for ParsedTask {
    fn from(task: Task) -> ParsedTask {
        let parsed_deadline = match &task.deadline {
            None => String::from("No deadline").cyan().to_string(),
            Some(d) => d.format("%Y-%m-%d %H:%M").to_string(),
        };

        let parsed_status = match task.status() {
            TaskStatus::Completed => String::from("Completed").green().to_string(),
            TaskStatus::Pending   => String::from("Pending").yellow().to_string(),
            TaskStatus::Overdue   => String::from("Overdue").red().to_string(),
        };

        ParsedTask {
            description: task.description,
            deadline: parsed_deadline,
            status: parsed_status
        }
    }
}


impl From<ParsedTask> for Vec<String> {
    fn from(pt: ParsedTask) -> Vec<String> {
        return vec![pt.description, pt.deadline, pt.status];

    }
}
