use clap::{Parser,Subcommand};

#[derive(Parser)]
#[command(name = "tsl")]
#[command(about = "(t)a(s)k (l)ist - Very simple CLI task manager")]

pub struct CLI {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create new task
    Add {
        /// Task text
        name: String,
        /// Optional: deadline
        #[arg(short,long)]
        deadline: Option<String>,
    },
    /// List currently pending tasks
    List,
    /// Mark task as finished
    Finish {
        index: usize,
    },
    /// Mark multiple tasks as finished
    FinishMany {
        /// Indices of tasks to mark as finished
        indices: Vec<usize>,
    },
    /// Delete task from database
    Delete {
        index: usize,
    },
}

