use chrono::Utc;

mod task;
use task::Task;

mod tasklist;
use tasklist::TaskList;
 
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
            task_list.add(
                Task::new(name, None)
            );
            task_list.save(&path);
        }

        Commands::Finish { index } => {
            task_list.finish(index);
            task_list.save(&path);
        }
    }
}
