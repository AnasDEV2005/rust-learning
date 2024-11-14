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


    // Iterate through each character in the input string
    for i in input.chars() {
        if is_int(i) {
            // Add number characters to num1 or num2
            if operator == ' ' {
                num1.push(i); // Add to num1 until we encounter an operator
            } else {
                num2.push(i); // Add to num2 after encountering an operator
            }
        } else if "+-/*".contains(i) {
            operator = i; // Set operator as the first operator found
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
        '+' => x + y,
        '-' => x - y,
        '*' => x * y,
        '/' => {
            if y != 0 {
                x / y
            } else {
                println!("Error: Division by zero.");
                0 // Return 0 or handle error as needed
            }
        }
        _ => {
            println!("Invalid operator.");
            0
        }
    }
}

