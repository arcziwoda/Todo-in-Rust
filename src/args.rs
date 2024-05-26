use clap::{Args, Parser, Subcommand};

/// Simple todo list app
#[derive(Parser, Debug)]
pub struct TodoArgs {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Show all tasks
    Show(ShowCommand),
    /// Add a new task
    Add(AddCommand),
    /// Remove a task
    Remove(RemoveCommand),
    /// Complete a task
    Complete(CompleteCommand),
    /// Uncomplete a task
    Uncomplete(UncompleteCommand),
}

#[derive(Args, Debug)]
pub struct ShowCommand {}

#[derive(Args, Debug)]
pub struct AddCommand {
    /// The name of the task
    pub name: String,
}

#[derive(Args, Debug)]
pub struct RemoveCommand {
    /// The ID of the task
    pub id: u32,
}

#[derive(Args, Debug)]
pub struct CompleteCommand {
    /// The ID of the task
    pub id: u32,
}

#[derive(Args, Debug)]
pub struct UncompleteCommand {
    /// The ID of the task
    pub id: u32,
}
