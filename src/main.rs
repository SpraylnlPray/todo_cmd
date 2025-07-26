use std::{
    cell::RefCell, io::{self, stdout, Write}, process::exit, sync::Arc
};
use todolib::{
    action::{action_complete_todo, action_create_todo, action_delete_todo, action_edit_todo, action_list_todos, Action},
    Todos,
};

fn main() {
    let todos: Todos = Arc::new(RefCell::new(Vec::new()));
    loop {
        std::process::Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .unwrap_or_else(|status| {
                println!("An error occurred: {}, exit.", status.to_string());
                exit(-1);
            });

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

        let mut buffer: String = String::new();
        buffer.clear();
        let stdin = io::stdin();
        match stdin.read_line(&mut buffer) {
            Ok(_) => (),
            Err(err) => {
                println!("Error when reading input: {}, exit.", err.to_string());
                exit(-1);
            }
        };

        let action = Action::from(buffer);
        match action {
            Action::Create => action_create_todo(todos.clone()),
            Action::Edit => action_edit_todo(todos.clone()),
            Action::Delete => action_delete_todo(todos.clone()),
            Action::List => action_list_todos(todos.clone()),
            Action::Complete => action_complete_todo(todos.clone()),
            Action::Exit => {
                println!("Exiting, Bye!");
                exit(0);
            }
            Action::Invalid => {
                println!("Invalid action");
            }
        }
    }
}
