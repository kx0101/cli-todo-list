use std::io;

fn load_options() -> Vec<String> {
    let options = [
        "Add a new todo item",
        "View all todo items",
        "Mark a todo item as complete",
        "Mark a todo item as incomplete",
        "Edit a todo item",
        "Delete a todo item",
        "Quit",
    ];

    return options.iter().map(|option| option.to_string()).collect();
}

fn add_todo(todos: &mut Vec<Todo>) {
    let mut title = String::new();

    loop {
        println!("Enter the title");
        io::stdin()
            .read_line(&mut title)
            .expect("Failed to read line");

        title = title.trim().to_string();

        if !title.is_empty() {
            if title.len() > 50 {
                println!("Title must be 50 characters or less.");
                title.clear();
                continue;
            }

            break;
        } else {
            println!("Title cannot be empty.");
            continue;
        }
    }

    let mut description = String::new();

    loop {
        println!("Enter the description");

        io::stdin()
            .read_line(&mut description)
            .expect("Failed to read line");

        description = description.trim().to_string();

        if !description.is_empty() {
            if description.len() > 250 {
                println!("Description must be 250 characters or less.");
                description.clear();
                continue;
            }

            break;
        } else {
            println!("Description cannot be empty.");
            continue;
        }
    }

    let todo = Todo {
        title,
        description,
        completed: false,
    };

    todos.push(todo);
}

fn view_todos(todos: &Vec<Todo>) {
    if todos.is_empty() {
        println!("No todos found.");
        return;
    } else {
        for (i, todo) in todos.iter().enumerate() {
            println!(
                "{}. {} - {} [{}]",
                i + 1,
                todo.title,
                todo.description,
                if todo.completed { "x" } else { " " }
            );
        }
    }
}

fn mark_todo_complete(todos: &mut Vec<Todo>) {
    if todos.is_empty() {
        println!("You have no todos saved");
        return;
    }

    let mut choice = String::new();

    loop {
        println!("Which one do you want to mark as completed");
        println!("\n");

        view_todos(todos);

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim().parse() {
            Ok::<usize, _>(index) => {
                if index == 0 || index > todos.len() {
                    println!("Invalid index, please try again");
                    choice.clear();
                    continue;
                }

                if todos[index - 1].completed {
                    println!("This is already marked as completed");
                    choice.clear();
                    continue;
                }

                todos[index - 1].completed = true;

                println!("\n");
                println!("Todo marked as done");
                break;
            }
            Err(_) => {
                println!("Invalid input, please try again");
                continue;
            }
        }
    }
}

fn mark_todo_incomplete() {
    println!("mark_todo_incomplete function");
}

fn edit_todo() {
    println!("edit_todo function");
}

fn delete_todo() {
    println!("delete_todo function");
}

struct Todo {
    title: String,
    description: String,
    completed: bool,
}

fn main() {
    println!("Welcome to your todo list!");

    let options = load_options();
    let mut todos: Vec<Todo> = Vec::new();
    let mut choice: u8 = 0;

    while choice != 9 {
        println!("\n");
        println!("What would you like to do?");
        println!("\n");
        println!("\n");
        for (index, option) in options.iter().enumerate() {
            println!("{}. {}", index + 1, option);
        }

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        choice = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please try again");
                continue;
            }
        };

        println!("\n");
        println!("\n");

        match choice {
            1 => add_todo(&mut todos),
            2 => view_todos(&todos),
            3 => mark_todo_complete(&mut todos),
            4 => mark_todo_incomplete(),
            5 => edit_todo(),
            6 => delete_todo(),
            7 => println!("Goodbye!"),
            _ => println!("Invalid input, please try again"),
        }
    }
}
