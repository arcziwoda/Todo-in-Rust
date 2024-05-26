use crate::args::TodoArgs;
use crate::io::{read_todo, write_todo};
use crate::todo::run;
use clap::Parser;

mod args;
mod io;
mod task;
mod todo;
mod utils;

fn main() {
    let args = TodoArgs::parse();
    let mut todo = read_todo().unwrap();
    run(&mut todo, args);
    write_todo(&todo).unwrap();
}
