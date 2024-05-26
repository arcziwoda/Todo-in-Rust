use crate::task::Task;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use dirs::home_dir;


pub fn read_todo() -> Result<Vec<Task>, Box<dyn Error>> {
    let todo_path = get_todo_path()?;
    if !todo_path.exists() {
        create_todo_file()?;
    }
    let file = File::open(todo_path)?;
    let reader = BufReader::new(file);
    let todo: Vec<Task> = serde_json::from_reader(reader)?;
    Ok(todo)
}

pub fn write_todo(todo: &Vec<Task>) -> Result<(), Box<dyn Error>> {
    let todo_path = get_todo_path()?;
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(todo_path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, todo)?;
    Ok(())
}

fn create_todo_file() -> Result<(), Box<dyn Error>> {
    let todo_path = get_todo_path()?;
    if let Some(parent) = todo_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(todo_path)?;
    let writer = BufWriter::new(file);
    let tasks: Vec<Task> = Vec::new();
    serde_json::to_writer(writer, &tasks)?;
    Ok(())
}

fn get_todo_path() -> Result<PathBuf, Box<dyn Error>> {
    let home = home_dir().ok_or("Cannot find home directory")?;
    let todo_dir = home.join("todo");
    let todo_file = todo_dir.join("todo.json");
    Ok(todo_file)
}
