use std::io;

fn main() {
    // Print a prompt to the user
    println!("Enter an equation to calculate (e.g. 2 + 2):");

    // Read a line of input from the user
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Parse the input as an equation
    let equation: Vec<&str> = input.trim().split(' ').collect();

    // Ensure that the equation is in the correct format (e.g. "2 + 2")
    if equation.len() != 3 {
        println!("Invalid equation format");
        return;
    }

    // Extract the operands and operator from the equation
    let operand1: f32 = equation[0].parse().unwrap();
    let operator: &str = equation[1];
    let operand2: f32 = equation[2].parse().unwrap();

    // Perform the calculation and print the result
    let result = match operator {
        "+" => operand1 + operand2,
        "-" => operand1 - operand2,
        "*" => operand1 * operand2,
        "/" => operand1 / operand2,
        _ => {
            println!("Invalid operator");
            return;
        }
    };
    println!("Result: {}", result);

    // Ask for another calculation
    println!("Would you like to perform another calculation? (y/n)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if input.trim() == "y" {
        main();
    }

    // Exit the program
    println!("Goodbye!");

    // wait for user to press enter
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

}
