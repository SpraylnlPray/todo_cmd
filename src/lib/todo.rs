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