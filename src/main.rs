use std::{
    cell::RefCell,
    fs::File,
    io::{stdout, BufReader, Error, Write},
    process::exit,
    sync::Arc,
};
use todolib::{
    action::{self, Action},
    errors::SelectionError,
    get_input,
    todo::Todo,
    Todos,
};

fn store(todos: Todos) -> Result<(), Error> {
    let json = serde_json::to_string(&*todos.clone().borrow())?;
    std::fs::write("todos.json", json)?;
    return Ok(());
}

fn load() -> std::io::Result<Todos> {
    let todos: Todos = Arc::new(RefCell::new(Vec::new()));
    let file = File::open("todos.json")?;
    let buf_reader = BufReader::new(file);
    let mut todos_temp: Vec<Todo> = serde_json::from_reader(buf_reader)?;
    todos.borrow_mut().append(&mut todos_temp);
    return Ok(todos);
}

fn print_main() {
    println!("\n########################################");
    println!("############# TODO Manager #############");
    println!("########################################");
    println!("\nAvailable Actions:");
    println!("1. Create TODO");
    println!("2. Edit TODO");
    println!("3. Delete TODO");
    println!("4. List TODOs");
    println!("5. Complete TODO");
    println!("6. Exit");
    println!("");

    print!("Enter your action: ");
    let _ = stdout().flush(); // This is necessary, otherwise the text appears after the next println
}

fn execute_action(exit_app: &mut bool, todos: &Arc<RefCell<Vec<Todo>>>, action: Action) {
    if let Err(err) = match action {
        Action::Create => action::create_todo(todos.clone()),
        Action::Edit => action::edit_todo(todos.clone()),
        Action::Delete => action::delete_todo(todos.clone()),
        Action::List => action::list_todos(todos.clone()),
        Action::Complete => action::complete_todo(todos.clone()),
        Action::Exit => {
            *exit_app = true;
            Ok(())
        }
        Action::Invalid => Err(SelectionError("Invalid Selection".to_string()).into()),
    } {
        println!("{}", err.to_string());
        std::thread::sleep(core::time::Duration::from_secs(1));
    }
}

fn clean_console() {
    #[cfg(target_family = "windows")]
    {
        std::process::Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .unwrap_or_else(|status| {
                println!(
                    "An error occurred while clearing the screen: {}, exit.",
                    status.to_string()
                );
                exit(-1);
            });
    }
    #[cfg(target_family = "unix")]
    {
        std::process::Command::new("clear").status().unwrap_or_else(|status| {
            println!(
                "An error occurred while clearing the screen: {}, exit.",
                status.to_string()
            );
            exit(-1);
        });
    }
}

fn main() {
    let mut exit_app = false;
    let todos: Todos = match load() {
        Ok(todos) => todos,
        Err(err) => {
            println!("Error loading data from file: {}", err.to_string());
            Arc::new(RefCell::new(Vec::new()))
        }
    };

    while !exit_app {
        clean_console();
        print_main();

        let input = match get_input() {
            Ok(input) => input,
            Err(err) => {
                println!("Error when reading input: {}, exit.", err.to_string());
                exit(-1);
            }
        };

        let action = Action::from(input);
        execute_action(&mut exit_app, &todos, action);
    }

    println!("Storing to file....");
    if let Err(err) = store(todos) {
        println!("Error storing data to file: {}", err.to_string());
    }
    println!("Exiting, Bye!");
    std::process::exit(0);
}
