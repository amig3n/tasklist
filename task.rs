use chrono::{DateTime, Utc};
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct Task {
    name: String,
    completed: bool,
    deadline: Option<DateTime<Utc>>
}

impl Task {
    pub fn new(name: String, deadline: Option<DateTime<Utc>>) -> Task {
        return Task{
            name: name,
            completed: false,
            deadline: deadline
        }
    }
    
    pub fn show(&self) {
        let parsed_deadline: String = match &self.deadline {
            None => "No deadline".to_string(),
            Some(d) => d.format("%Y-%m-%d %H:%M").to_string(),
        };
        let parsed_status: String = match &self.completed {
            true => "Completed".to_string(),
            false => "Not completed".to_string(),
        };
        println!("{} | {} | {}", self.name, parsed_deadline, parsed_status);
    }
    
    pub fn finish(&mut self) {
        self.completed = true;
    }
}
