use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    fs::File,
    io::{self, Write},
};

fn load_options() -> Vec<Choice> {
    return vec![
        Choice::AddTodo,
        Choice::ViewTodos,
        Choice::MarkTodoComplete,
        Choice::MarkTodoIncomplete,
        Choice::EditTodo,
        Choice::DeleteTodo,
        Choice::WriteTodosInFile,
        Choice::Quit,
    ];
}

fn prompt_string(prompt: &str, min_len: usize, max_len: usize) -> Option<String> {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim().to_string();

        if input == "0" {
            return None;
        }

        if input.is_empty() {
            println!("Input cannot be empty.");
            continue;
        }

        if input.len() < min_len {
            println!("Input must be at least {} characters.", min_len);
            continue;
        }

        if input.len() > max_len {
            println!("Input must be at most {} characters.", max_len);
            continue;
        }

        return Some(input);
    }
}

fn prompt_index(prompt: &str, max_index: usize) -> Option<usize> {
    loop {
        let input = match prompt_string(prompt, 1, 2) {
            Some(input) => input,
            None => return None,
        };

        match input.parse::<usize>() {
            Ok(index) if index <= max_index => return Some(index),
            _ => println!("Invalid input, please try again"),
        }
    }
}

fn write_todos_to_file(todos: &Vec<Todo>) -> Result<(), String> {
    println!("Enter 0 if you want to get back to the menu");
    println!("\n");

    let input = match prompt_string("Choose file format: (1) JSON, (2) Plain text", 1, 2) {
        Some(input) => input,
        None => return Err("Invalid input, please try again".to_string()),
    };

    match input.as_str() {
        "1" => {
            let mut file = match File::create("todos.json") {
                Ok(file) => file,
                Err(e) => return Err(format!("Failed to create file: {}", e)),
            };

            let json_todos = serde_json::to_string_pretty(&todos).unwrap();
            file.write_all(json_todos.as_bytes()).unwrap();

            return Ok(());
        }
        "2" => {
            let mut file = match File::create("todos.txt") {
                Ok(file) => file,
                Err(e) => return Err(format!("Failed to create file: {}", e)),
            };

            for (i, todo) in todos.iter().enumerate() {
                writeln!(
                    file,
                    "{}. {} - {} [{}]",
                    i + 1,
                    todo.title,
                    todo.description,
                    if todo.completed { "x" } else { " " }
                );
            }
            return Ok(());
        }
        _ => return Err("Invalid input".to_string()),
    };
    return Ok(());
}

fn create_todo() -> Result<Todo, String> {
    println!("Enter 0 if you want to get back to the menu");
    println!("\n");

    let title = match prompt_string("Enter the title", 1, 50) {
        Some(input) => input,
        None => return Err("Cancelled".to_string()),
    };

    let description =
        prompt_string("Enter the description", 0, 250).unwrap_or_else(|| "".to_string());

    return Ok(Todo {
        title,
        description,
        completed: false,
    });
}

fn add_todo(todos: &mut Vec<Todo>) {
    match create_todo() {
        Ok(todo) => todos.push(todo),
        Err(error) => eprintln!("Cancelled creating new todo, {}", error),
    }
}

fn view_todos(todos: &Vec<Todo>) {
    if todos.is_empty() {
        println!("No todos found.");
        return;
    }

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

fn mark_todo(todos: &mut Vec<Todo>, is_complete: bool, prompt: &str, success_message: &str) {
    if todos.is_empty() {
        println!("You have no todos saved");
        return;
    }

    loop {
        view_todos(todos);

        let index = match prompt_index(prompt, todos.len()) {
            Some(index) => index,
            None => return,
        };

        if todos[index - 1].completed == is_complete {
            println!(
                "This todo is already marked as {}",
                if is_complete {
                    "completed"
                } else {
                    "incompleted"
                }
            );
            continue;
        }

        todos[index - 1].completed = is_complete;

        println!();
        println!("{}", success_message);
        return;
    }
}

fn mark_todo_complete(todos: &mut Vec<Todo>) {
    mark_todo(
        todos,
        true,
        "Which one do you want to mark as completed? Enter 0 if you want to get back to the menu",
        "Todo marked as done",
    );
}

fn mark_todo_incomplete(todos: &mut Vec<Todo>) {
    mark_todo(
        todos,
        false,
        "Which one do you want to mark as completed? Enter 0 if you want to get back to the menu",
        "Todo marked as incomplete",
    );
}

fn edit_todo(todos: &mut Vec<Todo>) {
    if todos.is_empty() {
        println!("You have no todos saved");
        return;
    }

    loop {
        view_todos(todos);

        let index = match prompt_index(
            "Which one do you want to edit, Enter 0 if you want to get back to the menu",
            todos.len(),
        ) {
            Some(index) => index,
            None => return,
        };

        match index {
            0 => return,
            index => match create_todo() {
                Ok(todo) => {
                    todos[index - 1].title = todo.title;
                    todos[index - 1].description = todo.description;
                    println!("\n");
                    println!("Todo is edited!");
                    return;
                }
                Err(e) => {
                    if e == "Cancelled" {
                        return;
                    } else {
                        println!("Failed to create todo: {}", e);
                    }
                }
            },
        }
    }
}

fn delete_todo(todos: &mut Vec<Todo>) {
    if todos.is_empty() {
        println!("You have no todos saved");
        return;
    }

    loop {
        view_todos(todos);

        let index = match prompt_index(
            "Which one do you want to edit, Enter 0 if you want to get back to the menu",
            todos.len(),
        ) {
            Some(index) => index,
            None => return,
        };

        let confirm = prompt_string("Are you sure you want to delete this todo? (y/n)", 1, 1)
            .unwrap_or_else(|| "".to_string());

        let todo = &todos[index - 1];
        println!(
            "{}. {} - {} [{}]",
            index,
            todo.title,
            todo.description,
            if todo.completed { "x" } else { " " }
        );

        if confirm == "y" {
            todos.remove(index - 1);
            println!();
            println!("Todo deleted successfully");
            break;
        } else {
            println!("Todo not deleted");
        }
    }
}

enum FileFormat {
    Text,
    JSON,
}

enum Choice {
    AddTodo,
    ViewTodos,
    MarkTodoComplete,
    MarkTodoIncomplete,
    EditTodo,
    DeleteTodo,
    WriteTodosInFile,
    Quit,
}

impl Display for Choice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Choice::AddTodo => write!(f, "Add a new todo"),
            Choice::ViewTodos => write!(f, "View all todos"),
            Choice::MarkTodoComplete => write!(f, "Mark a todo as complete"),
            Choice::MarkTodoIncomplete => write!(f, "Mark a todo as incomplete"),
            Choice::EditTodo => write!(f, "Edit a todo"),
            Choice::DeleteTodo => write!(f, "Delete a todo"),
            Choice::WriteTodosInFile => write!(f, "Write todos in a file"),
            Choice::Quit => write!(f, "Quit the program"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    title: String,
    description: String,
    completed: bool,
}

fn main() {
    println!("Welcome to your todo list!");

    let options = load_options();
    let mut todos: Vec<Todo> = Vec::new();

    loop {
        println!("\n");
        println!("What would you like to do?");
        println!("\n");
        for (index, option) in options.iter().enumerate() {
            println!("{}. {}", index + 1, option);
        }

        println!("\n");
        println!("\n");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let choice = match input.trim() {
            "1" => Choice::AddTodo,
            "2" => Choice::ViewTodos,
            "3" => Choice::MarkTodoComplete,
            "4" => Choice::MarkTodoIncomplete,
            "5" => Choice::EditTodo,
            "6" => Choice::DeleteTodo,
            "7" => Choice::WriteTodosInFile,
            "8" => Choice::Quit,
            _ => {
                println!("Invalid input, please try again");
                continue;
            }
        };

        println!("\n");
        println!("\n");

        match choice {
            Choice::AddTodo => add_todo(&mut todos),
            Choice::ViewTodos => view_todos(&todos),
            Choice::MarkTodoComplete => mark_todo_complete(&mut todos),
            Choice::MarkTodoIncomplete => mark_todo_incomplete(&mut todos),
            Choice::EditTodo => edit_todo(&mut todos),
            Choice::DeleteTodo => delete_todo(&mut todos),
            Choice::WriteTodosInFile => match write_todos_to_file(&todos) {
                Ok(_) => println!("Succesffully wrote todos to file"),
                Err(e) => eprintln!("Error writing todos to file: {}", e),
            },
            Choice::Quit => {
                println!("Goodbye!");
                break;
            }
        }
    }
}
