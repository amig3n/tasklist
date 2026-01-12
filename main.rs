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

fn main() {
    let cli = CLI::parse();

    let path: &str = "/home/andrzej/tasklist.json";
    let mut task_list = TaskList::load(&path);

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
                        eprintln!("Invalid deadline {}: {}", d, e);
                        return;
                    }
                },
                None => None
            };
            
            task_list.add(
                Task::new(name, prepared_deadline)
            );
            task_list.save(&path);
        }

        Commands::Finish { index } => {
            task_list.finish(index);
            task_list.save(&path);
        }
    }
}
