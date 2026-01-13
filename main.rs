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

fn main() {

    // obtain path to tasklist file
    let path = match env::home_dir() {
        Some(path) => path.join("tasklist.json"),
        None => {
            eprintln!("fatal: Cannot determine home directory path");
            return;
        },
    };

    // TODO handle automatic creation of tasklist file if not present
    let mut task_list = match TaskList::load(&path) {
        Ok(tl) => tl,
        Err(_) => {
            // create blank tasklist
            eprintln!("warning: Unable to load existing tasklist, creating a new one.");
            TaskList::new()
        }
    };
        
    // handle CLI commands
    let cli = CLI::parse();
    match cli.command {
        Commands::List => {
            task_list.show();
        }

        Commands::Add {name,deadline} => {
            // parse deadline if provided
            
            let prepared_deadline = match deadline {
                Some(d) => match parse_deadline(&d) {
                    Ok(dt) => Some(dt),
                    Err(e) => {
                        eprintln!("Invalid deadline \"{}\": {}", d, e);
                        return;
                    }
                },
                None => None
            };
            
            task_list.add(
                Task::new(name, prepared_deadline)
            );

            match task_list.save(&path) {
                Ok(_) => return,
                Err(_) => {
                    eprintln!("Unable to save the tasklist");
                    return
                }
            }
        }

        Commands::Finish { index } => {
            task_list.finish(index).expect("Unable to finish the task: invalid index");
            match task_list.save(&path) {
                Ok(_) => return,
                Err(_) => {
                    eprintln!("Unable to save the tasklist");
                    return
                }
            }
        }

        Commands::Delete { index } => {
            task_list.delete(index);
            match task_list.save(&path) {
                Ok(_) => return,
                Err(_) => {
                    eprintln!("Unable to save the tasklist");
                    return
                }
            }
        }
    }
}
