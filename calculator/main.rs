use std::io::stdin;
use std::any::type_name;


fn main() {
    calculator_run();
}

fn calculator_run() {
    println!("Enter an operation formatted like so: (x+y)");
    let input = get_user_input();
    let operation = seperate_operation(input);
    println!("{}", calculate(operation));
}

fn get_user_input() -> String {
    let mut read = String::new();
    stdin()
      .read_line(&mut read);
    return read;
}

fn is_int(c: char) -> bool {
    c.is_digit(10)
}

fn seperate_operation(input: String) -> (String, char, String) {
    let mut operator = ' ';
    let mut num1: String = String::new();
    let mut num2: String = String::new();

    for i in input.chars() {
        if is_int(i) {
            if operator == ' ' {
                num1.push(i); 
            } else {
                num2.push(i); 
            }
        } else if "+-/*".contains(i) {
            operator = i; 
        }
    }
    (num1, operator, num2)
}

fn calculate(operation: (String, char, String)) -> i32 {
    // Try parsing the num1 and num2 from strings to integers
    let x: i32 = operation.0.parse().unwrap_or(0); // Default to 0 on error
    let y: i32 = operation.2.parse().unwrap_or(0); // Default to 0 on error

    // Perform the calculation based on the operator
    match operation.1 {
        '+' => x + y, // if operation.1 = '+' return x_ y
        '-' => x - y, // same idea
        '*' => x * y,
        '/' => {
            if y != 0 {
                x / y
            } else {
                println!("Error: Division by zero.");
                0 // Return 0 or handle error as needed
            }
        }
        _ => { // other error
            println!("Invalid operator.");
            0
        }
    }
}

