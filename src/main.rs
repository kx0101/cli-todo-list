use std::{fmt::Display, io};

fn load_options() -> Vec<Choice> {
    return vec![
        Choice::AddTodo,
        Choice::ViewTodos,
        Choice::MarkTodoComplete,
        Choice::MarkTodoIncomplete,
        Choice::EditTodo,
        Choice::DeleteTodo,
        Choice::Quit,
    ];
}

fn create_todo() -> Option<Todo> {
    let mut title = String::new();

    println!("Enter 0 if you want to get back to the menu");
    println!("\n");

    loop {
        println!("Enter the title");
        io::stdin()
            .read_line(&mut title)
            .expect("Failed to read line");

        title = title.trim().to_string();

        match title.parse::<usize>() {
            Ok(index) if index == 0 => return None,
            Ok(_) => {}
            Err(_) => {}
        }

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

    return Some(Todo {
        title,
        description,
        completed: false,
    });
}

fn add_todo(todos: &mut Vec<Todo>) {
    match create_todo() {
        Some(todo) => todos.push(todo),
        None => println!("Cancelled creating new todo"),
    }
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
        println!("Which one do you want to mark as complete");
        println!("\n");
        println!("Enter 0 if you want to get back to the menu");
        println!("\n");

        view_todos(todos);

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim().parse::<usize>() {
            Ok(0) => return,
            Ok(index) if index > todos.len() => {
                println!("Invalid index, please try again");
                choice.clear();
                continue;
            }
            Ok(index) => {
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

fn mark_todo_incomplete(todos: &mut Vec<Todo>) {
    if todos.is_empty() {
        println!("You have no todos saved");
        return;
    }

    let mut choice = String::new();

    loop {
        println!("Which one do you want to mark as incomplete");
        println!("\n");
        println!("Enter 0 if you want to get back to the menu");
        println!("\n");

        view_todos(todos);

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim().parse::<usize>() {
            Ok(0) => return,
            Ok(index) if index > todos.len() => {
                println!("Invalid index, please try again");
                choice.clear();
                continue;
            }
            Ok(index) => {
                if !todos[index - 1].completed {
                    println!("This is already marked as incompleted");
                    choice.clear();
                    continue;
                }

                todos[index - 1].completed = false;

                println!("\n");
                println!("Todo marked as incomplete");
                break;
            }
            Err(_) => {
                println!("Invalid input, please try again");
                continue;
            }
        }
    }
}

fn edit_todo(todos: &mut Vec<Todo>) {
    if todos.is_empty() {
        println!("You have no todos saved");
        return;
    }

    let mut choice = String::new();

    loop {
        println!("Which one do you want to edit");
        println!("\n");
        println!("Enter 0 if you want to get back to the menu");
        println!("\n");

        view_todos(todos);

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim().parse::<usize>() {
            Ok(0) => return,
            Ok(index) if index > todos.len() => {
                println!("Invalid index, please try again");
                choice.clear();
                continue;
            }
            Ok(index) => {
                match create_todo() {
                    Some(todo) => {
                        todos[index - 1].title = todo.title;
                        todos[index - 1].description = todo.description;
                    }
                    None => println!("Failed"),
                };

                println!("\n");
                println!("Todo is edited!");
                break;
            }
            Err(_) => {
                println!("Invalid input, please try again");
                continue;
            }
        }
    }
}

fn delete_todo(todos: &mut Vec<Todo>) {
    if todos.is_empty() {
        println!("You have no todos saved");
        return;
    }

    let mut choice = String::new();

    loop {
        println!("Which one do you want to delete");
        println!("\n");
        println!("Enter 0 if you want to get back to the menu");
        println!("\n");

        view_todos(todos);

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim().parse::<usize>() {
            Ok(0) => return,
            Ok(index) if index > todos.len() => {
                println!("Invalid index, please try again");
                choice.clear();
                continue;
            }
            Ok(index) => {
                println!("Are you sure you want to delete the following todo? (y/n)");
                let todo = &todos[index - 1];
                println!(
                    "{}. {} - {} [{}]",
                    index,
                    todo.title,
                    todo.description,
                    if todo.completed { "x" } else { " " }
                );

                let mut confirm = String::new();
                io::stdin()
                    .read_line(&mut confirm)
                    .expect("Failed to read line");

                if confirm.trim().to_lowercase() != "y" {
                    println!("Todo not deleted");
                    return;
                }

                todos.remove(index - 1);

                println!("\n");
                println!("Todo deleted successfully!");
                break;
            }
            Err(_) => {
                println!("Invalid input, please try again");
                continue;
            }
        }
    }
}

enum Choice {
    AddTodo,
    ViewTodos,
    MarkTodoComplete,
    MarkTodoIncomplete,
    EditTodo,
    DeleteTodo,
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
            Choice::Quit => write!(f, "Quit the program"),
        }
    }
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
            "7" => Choice::Quit,
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
            Choice::Quit => {
                println!("Goodbye!");
                break;
            }
        }
    }
}
