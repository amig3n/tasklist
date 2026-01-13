use chrono::Utc;

mod task;
use task::Task;

mod tasklist;
use tasklist::TaskList;

mod parse_date;
use parse_date::parse_deadline;
 
mod cli;
use cli::{CLI, Commands};
use clap::Parser;

use std::env;
use std::path::Path;

fn run() -> Result<(), String> {
    // obtain path to tasklist file
    let path = match env::home_dir() {
        Some(path) => path.join("tasklist.json"),
        None => {
            return Err("Cannot determine home directory path".to_string());
        },
    };

    // TODO handle automatic creation of tasklist file if not present
    let mut task_list = match TaskList::load(&path) {
        Ok(tl) => tl,
        Err(_) => {
            // Non-fatal error -> if occurs, continue
            eprintln!("warning: Unable to load existing tasklist, creating a new one.");
            TaskList::new()
            // TODO: make sure that this file will be saved, otherwise - circular problem
        }
    };
        
    // handle CLI commands
    let cli = CLI::parse();
    match cli.command {
        Commands::List => {
            task_list.show();
            Ok(())
        }

        Commands::Add {name,deadline} => {
            // parse deadline if provided
            match deadline {
                Some(d) => { 
                    task_list.add(
                        Task::new(
                            name, 
                            Some(parse_deadline(&d)?)
                        )
                    );
                },
                None => {
                    task_list.add(
                        Task::new(name, None)
                    );
                },
            };
            
            task_list.save(&path)?;
            Ok(())
        }

        Commands::Finish { index } => {
            task_list.finish(index)?;
            task_list.save(&path)?;
            Ok(())
        }

        Commands::Delete { index } => {
            task_list.delete(index)?;
            task_list.save(&path)?;
            Ok(())
        }
    }
}

fn main() {
    match run() {
        Ok(()) => (),
        Err(e) => {
            eprintln!("Error: {}", e);
            return
        }
    }
}
