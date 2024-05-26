use crate::args::{Command, TodoArgs};
use crate::task::Task;
use crate::utils::generate_task_id;

pub fn run(todo: &mut Vec<Task>, args: TodoArgs) {
    match args.command {
        Command::Show(_) => show_todo(todo),
        Command::Add(add) => {
            add_task(todo, &add.name);
            show_todo(todo);
        }
        Command::Remove(remove) => {
            remove_task(todo, remove.id);
            show_todo(todo);
        }
        Command::Complete(complete) => {
            complete_task(todo, complete.id);
            show_todo(todo);
        }
        Command::Uncomplete(uncomplete) => {
            uncomplete_task(todo, uncomplete.id);
            show_todo(todo);
        }
    }
}

fn show_todo(todo: &Vec<Task>) {
    if todo.is_empty() {
        println!("No tasks");
        return;
    }
    let mut sorted_todo = todo.clone();
    sorted_todo.sort_by_key(|task| task.id());
    for task in sorted_todo {
        let status = if task.is_completed() { "+" } else { "-" };
        println!("{} [{}] - {}", task.id(), status, task.name());
    }
}

fn get_task(todo: &mut Vec<Task>, id: u32) -> Result<&mut Task, &str> {
    for task in todo {
        if task.id() == id {
            return Ok(task);
        }
    }
    Err("Task not found")
}

fn add_task(todo: &mut Vec<Task>, name: &str) {
    let id = generate_task_id(todo);
    let task = Task::new(id, name.to_string());
    todo.push(task);
}

fn remove_task(todo: &mut Vec<Task>, id: u32) {
    todo.retain(|task| task.id() != id);
}

fn complete_task(todo: &mut Vec<Task>, id: u32) {
    if let Ok(task) = get_task(todo, id) {
        task.complete();
    }
}

fn uncomplete_task(todo: &mut Vec<Task>, id: u32) {
    if let Ok(task) = get_task(todo, id) {
        task.uncomplete();
    }
}
