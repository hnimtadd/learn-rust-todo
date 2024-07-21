use std::collections::HashMap;
use std::env;
use std::io::ErrorKind;

// TODO: move this struct into sub package
struct Todo {
    todos: HashMap<String, bool>,
}

impl Todo {
    fn insert(&mut self, todo: String) -> Result<(), std::io::Error> {
        // check if the todo is existed
        if self.todos.contains_key(&todo) {
            return Err(std::io::Error::new(
                ErrorKind::InvalidInput,
                "duplicated todo",
            ));
        }
        // Insert todo into the attributes `todos` of this struct, false mean
        // the todo is not completed.
        self.todos.insert(todo, false);
        return Ok({});
    }
    fn verbose(&mut self) {
        for (todo, status) in &self.todos {
            println!("TODO: {} - Done: {}", todo, status);
        }
    }
}

fn main() {
    let program_name = env::args().nth(0).expect("Failed to get the program name");
    let action = env::args().nth(1).expect("Please specify the action");
    let item = String::from(env::args().nth(2).expect("Please specify an item"));
    let mut todo: Todo = Todo {
        todos: HashMap::new(),
    };
    println!(
        "Running {}, with action {}, item {}",
        program_name, action, item
    );
    let _ = todo.insert(item).expect("failed to insert todo");

    todo.verbose()
}
