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

use std::fmt;

#[derive(Debug)]
pub enum AppError {
    TasklistError(tasklist::TaskListError),
    HomedirError,
    ParseDeadlineError(parse_date::DeadlineParseError),
    GeneralError(String),

}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::TasklistError(e) => write!(f, "Tasklist error: {}", e),
            AppError::GeneralError(msg) => write!(f, "General application error: {}", msg),
            AppError::HomedirError => write!(f, "Could not determine home directory"),
            AppError::ParseDeadlineError(e) => write!(f, "Deadline parse error: {}", e),
        }
    }
}

impl std::error::Error for AppError {}

impl From<tasklist::TaskListError> for AppError {
    fn from(err: tasklist::TaskListError) -> Self {
        AppError::TasklistError(err)
    }
}

impl From<parse_date::DeadlineParseError> for AppError {
    fn from(err: parse_date::DeadlineParseError) -> Self {
        AppError::ParseDeadlineError(err)
    }
}

impl From<env::VarError> for AppError {
    fn from(_: env::VarError) -> Self {
        AppError::HomedirError
    }
}

fn run() -> Result<(), AppError> {
    // obtain path to tasklist file
    let path = match env::home_dir() {
        Some(path) => path.join("tasklist.json"),
        None => {
            return Err(AppError::HomedirError);
        },
    };

    // TODO handle automatic creation of tasklist file if not present
    let mut task_list = match TaskList::load(&path) {
        Ok(tl) => tl,
        Err(e) => {
            return Err(AppError::TasklistError(e))?;
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

        Commands::FinishMany { indices } => {
            // FIXME move this to tasklist module
            for index in indices {
                match task_list.finish(index) {
                    Ok(()) => (),
                    Err(e) => {
                        eprintln!("warning: Index {} -> {}", index, e);
                        return Ok(continue)
                    }
                };
            }
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
            match e {
                _ => eprintln!("Error: {}", e),
            }
        }
    }
}
