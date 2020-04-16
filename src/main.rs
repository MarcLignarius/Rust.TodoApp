use std::env;

struct TodoItem {
    name: String,
    completed: char
}

impl TodoItem {
    fn new(name: String) -> TodoItem {
        TodoItem {
            name: name,
            completed: ' '
        }
    }
}

struct TodoList {
    list: Vec<TodoItem>
}

impl TodoList {
    fn new() -> TodoList {
        TodoList {
            list: Vec::new()
        }
    }

    fn add_to_list(&mut self, name: String) {
        let todo_item = TodoItem::new(name);
        self.list.push(todo_item);
    }

    fn print(&self) {
        for item in &self.list {
            println!("[{}] - {}", item.completed, item.name);
        }
    }
}

enum Command {
    Get,
    Add(String)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut todo_list = TodoList::new();
    
    let command = match args[1].as_str() {
        "get" => Command::Get,
        "add" => Command::Add(args[2].clone()),
        _ => panic!("Invalid command: Use get or add.")
    };

    todo_list.add_to_list("Do the groceries".to_string());
    todo_list.add_to_list("Clean the gutters".to_string());


    match command {
        Command::Get => todo_list.print(),
        Command::Add(item) => {
            todo_list.add_to_list(item);
            todo_list.print();
        }
    } 
}
