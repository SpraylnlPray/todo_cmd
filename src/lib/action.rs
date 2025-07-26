use crate::{todo::Todo, Todos};
use core::time;
use std::{
    io::{self, stdout, Write},
    thread::sleep,
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
    sleep(time::Duration::from_secs(1));
}

#[cfg(test)]
fn action_sleep() {
    ()
}

#[cfg(not(test))]
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

#[cfg(test)]
static mut GET_INPUT_RETVALS: Vec<GetInputVal> = Vec::new();
static mut GET_INPUT_RETVALS_INDEX: usize = 0;

#[cfg(test)]
#[derive(PartialEq, Eq)]
enum GetInputValType {
    Error,
    String,
}

#[cfg(test)]
struct GetInputVal {
    input_type: GetInputValType,
    value: String,
}
#[cfg(test)]
impl GetInputVal {
    pub fn new(input_type: GetInputValType, value: String) -> Self {
        Self { input_type, value }
    }
}

#[cfg(test)]
fn get_input() -> Result<String, std::io::Error> {
    let input: &GetInputVal;
    unsafe {
        input = GET_INPUT_RETVALS.get(GET_INPUT_RETVALS_INDEX).unwrap();
        GET_INPUT_RETVALS_INDEX = GET_INPUT_RETVALS_INDEX + 1;
    }

    if input.input_type == GetInputValType::Error {
        return Err(std::io::Error::new(
            io::ErrorKind::InvalidInput,
            input.value.clone(),
        ));
    }
    return Ok(String::from(input.value.clone()));
}

fn print_input_label(label: &str) {
    print!("{label}");
    let _ = stdout().flush(); // This is necessary, otherwise the text appears after the next println
}

pub fn action_list_todos(todos: Todos) {
    println!("Your TODO list:\n");
    for todo in todos.borrow().iter() {
        println!("completed: {} | text: {}", todo.completed, todo.text);
    }

    println!();
    println!("Press enter key to return");
    let _ = get_input();
}

pub fn action_create_todo(todos: Todos) {
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

pub fn action_complete_todo(todos: Todos) {
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

pub fn action_delete_todo(todos: Todos) {
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

pub fn action_edit_todo(todos: Todos) {
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

// NOTE: These tests must be run with --test-threads=1!
#[cfg(test)]
mod tests {
    use std::{cell::RefCell, sync::Arc};
    // TODO: test error cases when error handling is implemented

    use super::*;

    fn setup_get_input(inputs: Vec<GetInputVal>) {
        unsafe {
            GET_INPUT_RETVALS_INDEX = 0;
            GET_INPUT_RETVALS = inputs;
        }
    }

    #[test]
    fn test_list_todos() {
        let vals = vec![GetInputVal::new(GetInputValType::String, "Foo".to_string())];
        setup_get_input(vals);
        let todos: Todos = Arc::new(RefCell::new(vec![
            Todo::new("first".to_string()),
            Todo::new("second".to_string()),
            Todo::new("third".to_string()),
        ]));

        action_list_todos(todos.clone());
        assert_eq!(todos.borrow().len(), 3);
    }

    #[test]
    fn test_create_todo() {
        let vals = vec![
            GetInputVal::new(GetInputValType::String, "Foo".to_string()),
            GetInputVal::new(GetInputValType::String, "Bar".to_string()),
            GetInputVal::new(GetInputValType::Error, "".to_string()),
        ];
        setup_get_input(vals);
        let todos: Todos = Arc::new(RefCell::new(vec![
            Todo::new("first".to_string()),
            Todo::new("second".to_string()),
            Todo::new("third".to_string()),
        ]));

        action_create_todo(todos.clone());
        assert_eq!(todos.borrow().len(), 4);
        assert_eq!(todos.borrow().get(3).unwrap().completed, false);
        assert_eq!(todos.borrow().get(3).unwrap().text, "Foo");

        action_create_todo(todos.clone());
        assert_eq!(todos.borrow().len(), 5);
        assert_eq!(todos.borrow().get(4).unwrap().completed, false);
        assert_eq!(todos.borrow().get(4).unwrap().text, "Bar");

        action_create_todo(todos.clone());
        assert_eq!(todos.borrow().len(), 5);
    }

    #[test]
    fn test_complete_todo() {
        let vals = vec![
            GetInputVal::new(GetInputValType::String, "1".to_string()),
            GetInputVal::new(GetInputValType::String, "3".to_string()),
        ];
        setup_get_input(vals);
        let todos: Todos = Arc::new(RefCell::new(vec![
            Todo::new("first".to_string()),
            Todo::new("second".to_string()),
            Todo::new("third".to_string()),
        ]));

        action_complete_todo(todos.clone());
        assert_eq!(todos.borrow().len(), 3);
        assert_eq!(todos.borrow().get(0).unwrap().completed, true);
        assert_eq!(todos.borrow().get(0).unwrap().text, "first");

        action_complete_todo(todos.clone());
        assert_eq!(todos.borrow().len(), 3);
        assert_eq!(todos.borrow().get(2).unwrap().completed, true);
        assert_eq!(todos.borrow().get(2).unwrap().text, "third");
    }

    #[test]
    fn test_delete_todo() {
        let vals = vec![
            GetInputVal::new(GetInputValType::String, "1".to_string()),
            GetInputVal::new(GetInputValType::String, "2".to_string()),
        ];
        setup_get_input(vals);
        let todos: Todos = Arc::new(RefCell::new(vec![
            Todo::new("first".to_string()),
            Todo::new("second".to_string()),
            Todo::new("third".to_string()),
        ]));

        action_delete_todo(todos.clone());
        assert_eq!(todos.borrow().len(), 2);
        assert!(todos
            .borrow()
            .iter()
            .any(|todo: &Todo| todo.text.eq("second")));
        assert!(todos
            .borrow()
            .iter()
            .any(|todo: &Todo| todo.text.eq("third")));

        action_delete_todo(todos.clone());
        assert_eq!(todos.borrow().len(), 1);
        assert_eq!(todos.borrow().get(0).unwrap().text, "third");
    }

    #[test]
    fn test_edit_todo() {
        let vals = vec![
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
        setup_get_input(vals);
        let todos: Todos = Arc::new(RefCell::new(vec![
            Todo::new("first".to_string()),
            Todo::new("second".to_string()),
            Todo::new("third".to_string()),
        ]));

        action_edit_todo(todos.clone());
        assert_eq!(todos.borrow().len(), 3);
        assert_eq!(todos.borrow().get(0).unwrap().completed, false);
        assert_eq!(todos.borrow().get(0).unwrap().text, "first edited");

        action_edit_todo(todos.clone());
        assert_eq!(todos.borrow().len(), 3);
        assert_eq!(todos.borrow().get(0).unwrap().completed, false);
        assert_eq!(todos.borrow().get(0).unwrap().text, "first edited again");

        action_edit_todo(todos.clone());
        assert_eq!(todos.borrow().len(), 3);
        assert_eq!(todos.borrow().get(0).unwrap().completed, true);
        assert_eq!(todos.borrow().get(0).unwrap().text, "first edited again");

        action_edit_todo(todos.clone());
        assert_eq!(todos.borrow().len(), 3);
        assert_eq!(todos.borrow().get(0).unwrap().completed, false);
        assert_eq!(todos.borrow().get(0).unwrap().text, "first edited again");
    }
}
