use crate::{
    errors::{ApplicationError, SelectionError}, get_input, todo::Todo, Todos
};
use std::io::{stdout, Write};

/*
    Open Points:
    - Nicer display for TODOs
*/

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

fn print_input_label(label: &str) {
    print!("{label}");
    let _ = stdout().flush(); // This is necessary, otherwise the text appears after the next println
}

fn list_todos_internal<F>(todos: Todos, mut get_input: F) -> Result<(), ApplicationError>
where
    F: FnMut() -> Result<String, std::io::Error>,
{
    println!("Your TODO list:\n");
    for todo in todos.borrow().iter() {
        println!("completed: {} | text: {}", todo.completed, todo.text);
    }

    println!();
    println!("Press enter key to return");
    let _ = get_input()?;
    Ok(())
}
pub fn list_todos(todos: Todos) -> Result<(), ApplicationError> {
    list_todos_internal(todos, get_input)
}

fn create_todo_internal<F>(todos: Todos, mut get_input: F) -> Result<(), ApplicationError>
where
    F: FnMut() -> Result<String, std::io::Error>,
{
    print_input_label("Enter new TODO: ");
    let input = get_input()?;

    let new_todo: Todo = Todo::new(input);
    todos.borrow_mut().push(new_todo);

    println!("Successfully added new todo!");
    action_sleep();
    Ok(())
}
pub fn create_todo(todos: Todos) -> Result<(), ApplicationError> {
    create_todo_internal(todos, get_input)
}

fn complete_todo_internal<F>(todos: Todos, mut get_input: F) -> Result<(), ApplicationError>
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
    let input = get_input()?;

    let number = input.parse::<usize>()?;

    if number == 0 || number > todos.borrow().len() {
        return Err(SelectionError(input).into());
    }

    if let Some(todo) = todos.borrow_mut().get_mut(number - 1) {
        todo.completed = true;
        println!("Successfully marked TODO as completed.");
    } else {
        return Err(ApplicationError(
            "Failed to mark TODO as completed.".to_string(),
        ));
    }
    
    action_sleep();
    return Ok(());
}
pub fn complete_todo(todos: Todos) -> Result<(), ApplicationError> {
    complete_todo_internal(todos, get_input)
}

fn delete_todo_internal<F>(todos: Todos, mut get_input: F) -> Result<(), ApplicationError>
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

    print_input_label("Enter TODO to delete: ");
    let input = get_input()?;

    let number = input.parse::<usize>()?;

    if number == 0 || number > todos.borrow().len() {
        return Err(SelectionError(input).into());
    }

    todos.borrow_mut().swap_remove(number - 1);
    println!("Successfully delete TODO.");
    action_sleep();
    
    return Ok(());
}
pub fn delete_todo(todos: Todos) -> Result<(), ApplicationError> {
    delete_todo_internal(todos, get_input)
}

fn edit_todo_internal<F>(todos: Todos, mut get_input: F) -> Result<(), ApplicationError>
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

    print_input_label("Enter TODO to edit: ");
    let input = get_input()?;

    let number = input.parse::<usize>()?;

    if number == 0 || number > todos.borrow().len() {
        return Err(SelectionError(input).into());
    }

    print_input_label("Would you like to edit the [T]ext or toggle the [C]ompleted state? ");
    let input = get_input()?;

    let mut todos_ref = todos.borrow_mut();
    let todo = match todos_ref.get_mut(number - 1) {
        Some(todo) => todo,
        None => {
            return Err(ApplicationError("Failed to get TODO".to_string()));
        }
    };
    match input.as_str() {
        "T" | "t" => {
            println!("Current text: {}", todo.text);
            print_input_label("Enter new text: ");
            let input = get_input()?;

            todo.text = input;
            println!("Successfully updated TODO. New text: {}", todo.text);
        }

        "C" | "c" => {
            todo.completed = !todo.completed;
            println!(
                "Successfully toggled completed state. New state: {}",
                todo.completed
            );
        }
        _ => {
            return Err(SelectionError("Invalid Selection".to_string()).into());
        }
    };

    action_sleep();
    return Ok(());
}

pub fn edit_todo(todos: Todos) -> Result<(), ApplicationError> {
    edit_todo_internal(todos, get_input)
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::{cell::RefCell, sync::Arc};
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
        let mock_inputs = vec![GetInputVal::new(GetInputValType::String, "".to_string()), GetInputVal::new(GetInputValType::Error, "".to_string())];
        let provider = MockInputProvider::new(mock_inputs);

        let todos: Todos = Arc::new(RefCell::new(vec![
            Todo::new("first".to_string()),
            Todo::new("second".to_string()),
            Todo::new("third".to_string()),
        ]));

        let res = list_todos_internal(todos.clone(), provider.get_fn());
        assert!(res.is_ok());
        assert_eq!(todos.borrow().len(), 3);

        let res = list_todos_internal(todos.clone(), provider.get_fn());
        assert!(res.is_err());
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

        let res = create_todo_internal(todos.clone(), provider.get_fn());
        assert_eq!(todos.borrow().len(), 4);
        assert_eq!(todos.borrow().get(3).unwrap().completed, false);
        assert_eq!(todos.borrow().get(3).unwrap().text, "Foo");
        assert!(res.is_ok());

        let res = create_todo_internal(todos.clone(), provider.get_fn());
        assert_eq!(todos.borrow().len(), 5);
        assert_eq!(todos.borrow().get(4).unwrap().completed, false);
        assert_eq!(todos.borrow().get(4).unwrap().text, "Bar");
        assert!(res.is_ok());

        let res = create_todo_internal(todos.clone(), provider.get_fn());
        assert_eq!(todos.borrow().len(), 5);
        assert!(res.is_err()); // Input Error
    }

    #[test]
    fn test_complete_todo() {
        let mock_inputs = vec![
            GetInputVal::new(GetInputValType::String, "1".to_string()),
            GetInputVal::new(GetInputValType::String, "3".to_string()),
            GetInputVal::new(GetInputValType::Error, "".to_string()),
            GetInputVal::new(GetInputValType::String, "4".to_string()),
        ];
        let provider = MockInputProvider::new(mock_inputs);

        let todos: Todos = Arc::new(RefCell::new(vec![
            Todo::new("first".to_string()),
            Todo::new("second".to_string()),
            Todo::new("third".to_string()),
        ]));

        let res = complete_todo_internal(todos.clone(), provider.get_fn());
        assert_eq!(todos.borrow().len(), 3);
        assert_eq!(todos.borrow().get(0).unwrap().completed, true);
        assert_eq!(todos.borrow().get(0).unwrap().text, "first");
        assert!(res.is_ok());

        let res = complete_todo_internal(todos.clone(), provider.get_fn());
        assert_eq!(todos.borrow().len(), 3);
        assert_eq!(todos.borrow().get(2).unwrap().completed, true);
        assert_eq!(todos.borrow().get(2).unwrap().text, "third");
        assert!(res.is_ok());

        let res = complete_todo_internal(todos.clone(), provider.get_fn());
        assert_eq!(todos.borrow().len(), 3);
        assert_eq!(todos.borrow().get(2).unwrap().completed, true);
        assert_eq!(todos.borrow().get(2).unwrap().text, "third");
        assert!(res.is_err()); // Input Error

        let res = complete_todo_internal(todos.clone(), provider.get_fn());
        assert_eq!(todos.borrow().len(), 3);
        assert_eq!(todos.borrow().get(2).unwrap().completed, true);
        assert_eq!(todos.borrow().get(2).unwrap().text, "third");
        assert!(res.is_err()); // Selection Error
    }

    #[test]
    fn test_delete_todo() {
        let mock_inputs = vec![
            GetInputVal::new(GetInputValType::String, "1".to_string()),
            GetInputVal::new(GetInputValType::String, "2".to_string()),
            GetInputVal::new(GetInputValType::Error, "".to_string()),
            GetInputVal::new(GetInputValType::String, "2".to_string()),
        ];
        let provider = MockInputProvider::new(mock_inputs);

        let todos: Todos = Arc::new(RefCell::new(vec![
            Todo::new("first".to_string()),
            Todo::new("second".to_string()),
            Todo::new("third".to_string()),
        ]));

        let res = delete_todo_internal(todos.clone(), provider.get_fn());
        assert!(res.is_ok());
        assert_eq!(todos.borrow().len(), 2);
        assert!(todos
            .borrow()
            .iter()
            .any(|todo: &Todo| todo.text.eq("second")));
        assert!(todos
            .borrow()
            .iter()
            .any(|todo: &Todo| todo.text.eq("third")));

        let res = delete_todo_internal(todos.clone(), provider.get_fn());
        assert!(res.is_ok());
        assert_eq!(todos.borrow().len(), 1);
        assert_eq!(todos.borrow().get(0).unwrap().text, "third");

        let res = delete_todo_internal(todos.clone(), provider.get_fn());
        assert!(res.is_err()); // Input Error
        assert_eq!(todos.borrow().len(), 1);
        assert_eq!(todos.borrow().get(0).unwrap().text, "third");

        let res = delete_todo_internal(todos.clone(), provider.get_fn());
        assert!(res.is_err()); // Selection Error
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
            GetInputVal::new(GetInputValType::Error, "".to_string()),
            GetInputVal::new(GetInputValType::Error, "4".to_string()),
            GetInputVal::new(GetInputValType::Error, "1".to_string()),
            GetInputVal::new(GetInputValType::Error, "f".to_string()),
        ];
        let provider = MockInputProvider::new(mock_inputs);

        let todos: Todos = Arc::new(RefCell::new(vec![
            Todo::new("first".to_string()),
            Todo::new("second".to_string()),
            Todo::new("third".to_string()),
        ]));

        let res = edit_todo_internal(todos.clone(), provider.get_fn());
        assert!(res.is_ok());
        assert_eq!(todos.borrow().len(), 3);
        assert_eq!(todos.borrow().get(0).unwrap().completed, false);
        assert_eq!(todos.borrow().get(0).unwrap().text, "first edited");

        let res = edit_todo_internal(todos.clone(), provider.get_fn());
        assert!(res.is_ok());
        assert_eq!(todos.borrow().len(), 3);
        assert_eq!(todos.borrow().get(0).unwrap().completed, false);
        assert_eq!(todos.borrow().get(0).unwrap().text, "first edited again");

        let res = edit_todo_internal(todos.clone(), provider.get_fn());
        assert!(res.is_ok());
        assert_eq!(todos.borrow().len(), 3);
        assert_eq!(todos.borrow().get(0).unwrap().completed, true);
        assert_eq!(todos.borrow().get(0).unwrap().text, "first edited again");

        let res = edit_todo_internal(todos.clone(), provider.get_fn());
        assert!(res.is_ok());
        assert_eq!(todos.borrow().len(), 3);
        assert_eq!(todos.borrow().get(0).unwrap().completed, false);
        assert_eq!(todos.borrow().get(0).unwrap().text, "first edited again");

        let res = edit_todo_internal(todos.clone(), provider.get_fn());
        assert!(res.is_err()); // Input Error
        assert_eq!(todos.borrow().len(), 3);
        assert_eq!(todos.borrow().get(0).unwrap().completed, false);
        assert_eq!(todos.borrow().get(0).unwrap().text, "first edited again");

        let res = edit_todo_internal(todos.clone(), provider.get_fn());
        assert!(res.is_err()); // Selection Error
        assert_eq!(todos.borrow().len(), 3);
        assert_eq!(todos.borrow().get(0).unwrap().completed, false);
        assert_eq!(todos.borrow().get(0).unwrap().text, "first edited again");

        let res = edit_todo_internal(todos.clone(), provider.get_fn());
        assert!(res.is_err()); // Selection Error
        assert_eq!(todos.borrow().len(), 3);
        assert_eq!(todos.borrow().get(0).unwrap().completed, false);
        assert_eq!(todos.borrow().get(0).unwrap().text, "first edited again");
    }
}
