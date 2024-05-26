use crate::task::Task;

pub fn generate_task_id(todo: &Vec<Task>) -> u32 {
    let ids: Vec<u32> = todo.iter().map(|task| task.id()).collect();
    let mut id = 1;
    while ids.contains(&id) {
        id += 1;
    }
    id
}

