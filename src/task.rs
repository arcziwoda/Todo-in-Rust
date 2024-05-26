use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    id: u32,
    name: String,
    completed: bool,
}

impl Task {
    pub fn new(id: u32, name: String) -> Task {
        Task {
            id,
            name,
            completed: false,
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }

    pub fn uncomplete(&mut self) {
        self.completed = false;
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }
}

