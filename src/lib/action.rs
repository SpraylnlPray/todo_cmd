use crate::{todo::Todo, Todos};
use std::{
    io::{self, stdout, Write},
};

pub enum Action {
    Create,
    Edit,
    Delete,
    List,
    Complete,
    Exit,
    Invalid,
}

impl From<String> for Action {
    fn from(mut val: String) -> Self {
        if val.len() < 1 {
            return Action::Invalid;
        }

        if let Some('\n') = val.chars().next_back() {
            val.pop();
        }

        if let Some('\r') = val.chars().next_back() {
            val.pop();
        }

        match val.as_str() {
            "1" => Action::Create,
            "2" => Action::Edit,
            "3" => Action::Delete,
            "4" => Action::List,
            "5" => Action::Complete,
            "6" => Action::Exit,
            _ => Action::Invalid,
        }
    }
}

#[cfg(not(test))]
fn action_sleep() {
    use core::time;
    use std::thread::sleep;
    sleep(time::Duration::from_secs(1));
}

#[cfg(test)]
fn action_sleep() {
    ()
}

fn get_input() -> Result<String, std::io::Error> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            if let Some('\n') = input.chars().next_back() {
                input.pop();
            }
            if let Some('\r') = input.chars().next_back() {
                input.pop();
            }
            return Ok(input);
        }
        Err(err) => Err(err),
    }
}

fn print_input_label(label: &str) {
    print!("{label}");
    let _ = stdout().flush(); // This is necessary, otherwise the text appears after the next println
}

fn list_todos_internal<F>(todos: Todos, mut get_input: F)
where
    F: FnMut() -> Result<String, std::io::Error>,
{
    println!("Your TODO list:\n");
    for todo in todos.borrow().iter() {
        println!("completed: {} | text: {}", todo.completed, todo.text);
    }

    println!();
    println!("Press enter key to return");
    let _ = get_input();
}
pub fn list_todos(todos: Todos) {
    list_todos_internal(todos, get_input);
}

fn create_todo_internal<F>(todos: Todos, mut get_input: F)
where
    F: FnMut() -> Result<String, std::io::Error>,
{
    print_input_label("Enter new TODO: ");
    let input = match get_input() {
        Ok(input) => input,
        Err(err) => {
            println!("Error reading input {err}");
            action_sleep();
            return;
        }
    };

    let new_todo: Todo = Todo::new(input);
    todos.borrow_mut().push(new_todo);

    println!("Successfully added new todo!");
    action_sleep();
}
pub fn create_todo(todos: Todos) {
    create_todo_internal(todos, get_input);
}

fn complete_todo_internal<F>(todos: Todos, mut get_input: F)
where
    F: FnMut() -> Result<String, std::io::Error>,
{
    println!("Your TODO list:\n");
    for (i, todo) in todos.borrow().iter().enumerate() {
        println!(
            "# {}: completed: {} | text: {}",
            i + 1,
            todo.completed,
            todo.text
        );
    }
    println!();

    print_input_label("Enter TODO to complete: ");
    let input = match get_input() {
        Ok(input) => input,
        Err(err) => {
            println!("Error reading input {err}, exit.");
            action_sleep();
            return;
        }
    };

    let number = match input.parse::<usize>() {
        Ok(num) => num,
        Err(err) => {
            println!("Error parsing number: {err}, exit.");
            action_sleep();
            return;
        }
    };

    if number == 0 || number > todos.borrow().len() {
        println!("Invalid number");
        action_sleep();
        return;
    }

    if let Some(todo) = todos.borrow_mut().get_mut(number - 1) {
        todo.completed = true;
        println!("Successfully marked TODO as completed.");
    } else {
        println!("Failed to mark TODO as completed.");
    }
    action_sleep();
}
pub fn complete_todo(todos: Todos) {
    complete_todo_internal(todos, get_input);
}

fn delete_todo_internal<F>(todos: Todos, mut get_input: F)
where
    F: FnMut() -> Result<String, std::io::Error>,
{
    println!("Your TODO list:\n");
    for (i, todo) in todos.borrow().iter().enumerate() {
        println!(
            "# {}: completed: {} | text: {}",
            i + 1,
            todo.completed,
            todo.text
        );
    }
    println!();

    print_input_label("Enter TODO to complete: ");
    let input = match get_input() {
        Ok(input) => input,
        Err(err) => {
            println!("Error reading input {err}, exit.");
            action_sleep();
            return;
        }
    };

    let number = match input.parse::<usize>() {
        Ok(num) => num,
        Err(err) => {
            println!("Error parsing number: {err}, exit.");
            action_sleep();
            return;
        }
    };

    if number == 0 || number > todos.borrow().len() {
        println!("Invalid number");
        action_sleep();
        return;
    }

    todos.borrow_mut().swap_remove(number - 1);
    println!("Successfully delete TODO.");
    action_sleep();
}
pub fn delete_todo(todos: Todos) {
    delete_todo_internal(todos, get_input);
}

fn edit_todo_internal<F>(todos: Todos, mut get_input: F)
where
    F: FnMut() -> Result<String, std::io::Error>,
{
    println!("Your TODO list:\n");
    for (i, todo) in todos.borrow().iter().enumerate() {
        println!(
            "# {}: completed: {} | text: {}",
            i + 1,
            todo.completed,
            todo.text
        );
    }
    println!();

    print_input_label("Enter TODO to complete: ");
    let input = match get_input() {
        Ok(input) => input,
        Err(err) => {
            println!("Error reading input {err}, exit.");
            action_sleep();
            return;
        }
    };

    let number = match input.parse::<usize>() {
        Ok(num) => num,
        Err(err) => {
            println!("Error parsing number: {err}, exit.");
            action_sleep();
            return;
        }
    };

    if number == 0 || number > todos.borrow().len() {
        println!("Invalid number");
        action_sleep();
        return;
    }

    print_input_label("Would you like to edit the [T]ext or toggle the [C]ompleted state? ");
    let input = match get_input() {
        Ok(input) => input,
        Err(err) => {
            println!("Error reading input {err}, exit.");
            action_sleep();
            return;
        }
    };

    match todos.borrow_mut().get_mut(number - 1) {
        Some(todo) => {
            match input.as_str() {
                "T" | "t" => {
                    println!("Current text: {}", todo.text);
                    print_input_label("Enter new text: ");
                    match get_input() {
                        Ok(input) => {
                            todo.text = input;
                            println!("Successfully updated TODO. New text: {}", todo.text);
                            action_sleep();
                            return;
                        }
                        Err(err) => {
                            println!("Error reading input {err}, exit.");
                            action_sleep();
                            return;
                        }
                    };
                }
                "C" | "c" => {
                    todo.completed = !todo.completed;
                    println!(
                        "Successfully toggled completed state. New state: {}",
                        todo.completed
                    );
                    action_sleep();
                    return;
                }
                _ => {
                    println!("Invalid action");
                    action_sleep();
                }
            };
        }
        None => {
            println!("Failed to get TODO");
            action_sleep();
            return;
        }
    };
}

pub fn edit_todo(todos: Todos) {
    edit_todo_internal(todos, get_input);
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::{cell::RefCell, sync::Arc};
    // TODO: test error cases when error handling is implemented
    use super::*;

    struct MockInputProvider {
        inputs: Rc<RefCell<Vec<GetInputVal>>>,
    }

    impl MockInputProvider {
        pub fn new(inputs: Vec<GetInputVal>) -> Self {
            Self {
                inputs: Rc::new(RefCell::new(inputs)),
            }
        }

        // Returns a function that returns the inputs passed to `new`
        pub fn get_fn(&self) -> impl FnMut() -> Result<String, std::io::Error> {
            let inputs = self.inputs.clone();
            move || {
                let mut queue = inputs.borrow_mut();
                if queue.is_empty() {
                    panic!("No more mock inputs available!")
                }
                let input = queue.remove(0);
                match input.input_type {
                    GetInputValType::Error => Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        input.value.clone(),
                    )),
                    GetInputValType::String => Ok(input.value.clone()),
                }
            }
        }
    }

    #[derive(PartialEq, Eq)]
    enum GetInputValType {
        Error,
        String,
    }

    struct GetInputVal {
        input_type: GetInputValType,
        value: String,
    }
    impl GetInputVal {
        pub fn new(input_type: GetInputValType, value: String) -> Self {
            Self { input_type, value }
        }
    }

    #[test]
    fn test_list_todos() {
        let mock_inputs = vec![GetInputVal::new(GetInputValType::String, "".to_string())];
        let provider = MockInputProvider::new(mock_inputs);

        let todos: Todos = Arc::new(RefCell::new(vec![
            Todo::new("first".to_string()),
            Todo::new("second".to_string()),
            Todo::new("third".to_string()),
        ]));

        list_todos_internal(todos.clone(), provider.get_fn());
        assert_eq!(todos.borrow().len(), 3);
    }

    #[test]
    fn test_create_todo() {
        let mock_inputs = vec![
            GetInputVal::new(GetInputValType::String, "Foo".to_string()),
            GetInputVal::new(GetInputValType::String, "Bar".to_string()),
            GetInputVal::new(GetInputValType::Error, "".to_string()),
        ];
        let provider = MockInputProvider::new(mock_inputs);

        let todos: Todos = Arc::new(RefCell::new(vec![
            Todo::new("first".to_string()),
            Todo::new("second".to_string()),
            Todo::new("third".to_string()),
        ]));

        create_todo_internal(todos.clone(), provider.get_fn());
        assert_eq!(todos.borrow().len(), 4);
        assert_eq!(todos.borrow().get(3).unwrap().completed, false);
        assert_eq!(todos.borrow().get(3).unwrap().text, "Foo");

        create_todo_internal(todos.clone(), provider.get_fn());
        assert_eq!(todos.borrow().len(), 5);
        assert_eq!(todos.borrow().get(4).unwrap().completed, false);
        assert_eq!(todos.borrow().get(4).unwrap().text, "Bar");

        create_todo_internal(todos.clone(), provider.get_fn());
        assert_eq!(todos.borrow().len(), 5);
    }

    #[test]
    fn test_complete_todo() {
        let mock_inputs = vec![
            GetInputVal::new(GetInputValType::String, "1".to_string()),
            GetInputVal::new(GetInputValType::String, "3".to_string()),
        ];
        let provider = MockInputProvider::new(mock_inputs);

        let todos: Todos = Arc::new(RefCell::new(vec![
            Todo::new("first".to_string()),
            Todo::new("second".to_string()),
            Todo::new("third".to_string()),
        ]));

        complete_todo_internal(todos.clone(), provider.get_fn());
        assert_eq!(todos.borrow().len(), 3);
        assert_eq!(todos.borrow().get(0).unwrap().completed, true);
        assert_eq!(todos.borrow().get(0).unwrap().text, "first");

        complete_todo_internal(todos.clone(), provider.get_fn());
        assert_eq!(todos.borrow().len(), 3);
        assert_eq!(todos.borrow().get(2).unwrap().completed, true);
        assert_eq!(todos.borrow().get(2).unwrap().text, "third");
    }

    #[test]
    fn test_delete_todo() {
        let mock_inputs = vec![
            GetInputVal::new(GetInputValType::String, "1".to_string()),
            GetInputVal::new(GetInputValType::String, "2".to_string()),
        ];
        let provider = MockInputProvider::new(mock_inputs);


        let todos: Todos = Arc::new(RefCell::new(vec![
            Todo::new("first".to_string()),
            Todo::new("second".to_string()),
            Todo::new("third".to_string()),
        ]));

        delete_todo_internal(todos.clone(), provider.get_fn());
        assert_eq!(todos.borrow().len(), 2);
        assert!(todos
            .borrow()
            .iter()
            .any(|todo: &Todo| todo.text.eq("second")));
        assert!(todos
            .borrow()
            .iter()
            .any(|todo: &Todo| todo.text.eq("third")));

        delete_todo_internal(todos.clone(), provider.get_fn());
        assert_eq!(todos.borrow().len(), 1);
        assert_eq!(todos.borrow().get(0).unwrap().text, "third");
    }

    #[test]
    fn test_edit_todo() {
        let mock_inputs = vec![
            GetInputVal::new(GetInputValType::String, "1".to_string()),
            GetInputVal::new(GetInputValType::String, "T".to_string()),
            GetInputVal::new(GetInputValType::String, "first edited".to_string()),
            GetInputVal::new(GetInputValType::String, "1".to_string()),
            GetInputVal::new(GetInputValType::String, "t".to_string()),
            GetInputVal::new(GetInputValType::String, "first edited again".to_string()),
            GetInputVal::new(GetInputValType::String, "1".to_string()),
            GetInputVal::new(GetInputValType::String, "C".to_string()),
            GetInputVal::new(GetInputValType::String, "1".to_string()),
            GetInputVal::new(GetInputValType::String, "C".to_string()),
        ];
        let provider = MockInputProvider::new(mock_inputs);

        let todos: Todos = Arc::new(RefCell::new(vec![
            Todo::new("first".to_string()),
            Todo::new("second".to_string()),
            Todo::new("third".to_string()),
        ]));

        edit_todo_internal(todos.clone(), provider.get_fn());
        assert_eq!(todos.borrow().len(), 3);
        assert_eq!(todos.borrow().get(0).unwrap().completed, false);
        assert_eq!(todos.borrow().get(0).unwrap().text, "first edited");

        edit_todo_internal(todos.clone(), provider.get_fn());
        assert_eq!(todos.borrow().len(), 3);
        assert_eq!(todos.borrow().get(0).unwrap().completed, false);
        assert_eq!(todos.borrow().get(0).unwrap().text, "first edited again");

        edit_todo_internal(todos.clone(), provider.get_fn());
        assert_eq!(todos.borrow().len(), 3);
        assert_eq!(todos.borrow().get(0).unwrap().completed, true);
        assert_eq!(todos.borrow().get(0).unwrap().text, "first edited again");

        edit_todo_internal(todos.clone(), provider.get_fn());
        assert_eq!(todos.borrow().len(), 3);
        assert_eq!(todos.borrow().get(0).unwrap().completed, false);
        assert_eq!(todos.borrow().get(0).unwrap().text, "first edited again");
    }
}
