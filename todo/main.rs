use std::error::Error;
use std::io::stdin;
use std::fs::{self, File};
use std::path::Path;
use std::process;
use csv::{ReaderBuilder, WriterBuilder};

fn main() {
    println!("       ||    COMMAND LINE  TO DO    ||");
    loop {
        run_todo();
    }
}


fn load_todos(savefile_path: &str) -> Vec<String> {
    println!("Loading task list...");
    if !Path::new(savefile_path).exists() {
        return Vec::new(); // If the file does not exist, return an empty vector
    }

    let mut todos_vec = Vec::new();
    let mut rdr = ReaderBuilder::new().has_headers(false).from_path(savefile_path).unwrap();

    // Read the CSV file and collect all rows into the vector
    for result in rdr.records() {
        let record = result.unwrap();
        if let Some(todo) = record.get(0) {
            todos_vec.push(todo.to_string());
        }
    }

    todos_vec
}

fn update_savefile(todos_vec: Vec<String>, savefile_path: &str) -> Result<(), Box<dyn Error>> {
    // if no save file create it and save
    // if save file exist, write to it the entered vec
    // Open or create the CSV file for writing
    let mut write = WriterBuilder::new().from_path(savefile_path)?;

    // Write the vector to the CSV file
    for todo in todos_vec {
        write.write_record(&[todo])?;  // Write each string as a separate row
    }

    write.flush()?;
    println!("   -- Updated List --");
    Ok(())
}


fn display_todos() {
    let mut loaded_todos: Vec<String> = load_todos("todos.csv");
    let mut n: u32 = 1;
    println!("n | ---  TASKS ----------------");
    println!("--|-----------------------------");
    if loaded_todos.is_empty() {
        println!(".");
        println!(" List is empty, add a new task lil bro.");
        println!(".");
    } else {
        for i in loaded_todos {
            println!("{n} | {i}");
        }
    }
    
}

fn add_input_todo(todos_vec: &mut Vec<String>) {
    println!("Type task to input: ");
    let mut new_todo: String = String::new();
    stdin()
        .read_line(&mut new_todo)
        .expect("Expected something..?");
    todos_vec.push(new_todo.trim().to_string());
}

fn remove_todo(todos_vec: &mut Vec<String>) {
    println!("Type index of task to remove: ");
    let mut todo_index: String = String::new();
    stdin()
        .read_line(&mut todo_index);
    let todo_i: usize = todo_index.trim().parse::<usize>().unwrap() - 1;  
    todos_vec.remove(todo_i);
}


fn ask_action() -> char {
    println!("Choose action - Add task/Remove task/Quit (a/r/q) :");

    let mut action_choice = String::new(); // Store input as a String
    stdin()
        .read_line(&mut action_choice)  // Read user input into a String
        .expect("Failed to read line");

    let action_choice = action_choice.trim().chars().next().unwrap_or('x'); 

    match action_choice {
        'a' | 'r' | 'q' => action_choice,
        _ => 'x',

    }
}


fn quit_program() {
    println!("Quitting program...");
    display_todos();
    process::exit(0);
}

fn run_todo() {
    println!("||   - Rust Cli Todo Program -   ||");
    println!("  -------------------------------   ");
    let mut loaded_todos: Vec<String> = load_todos("todos.csv");
    display_todos();
    let choice: char = ask_action();
    if choice == 'a' {
        add_input_todo(&mut loaded_todos);
        update_savefile(loaded_todos, "todos.csv");
    } else if choice == 'r' {
        remove_todo(&mut loaded_todos);
        update_savefile(loaded_todos, "todos.csv");
    } else if choice == 'q' {
        quit_program();
    } else {
        println!("error, restarting");
        return run_todo();
    }
}