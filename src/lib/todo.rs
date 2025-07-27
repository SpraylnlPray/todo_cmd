use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub text: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(text: String) -> Self {
        Self {
            text,
            completed: false,
        }
    }
}