use std::env::{args, Args};

fn main() {
    // Typing the args with the Struct Args
    // Making Variable args mutable
    let mut args: Args = args();

    // nth() Method an Iterator from the Arguments Array
    // Unwrapping to get Value from Enum
    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    // Converting into Floats
    let first_number: f32 = first.parse().unwrap();
    // Using Turbofish Syntax
    let second_number = second.parse::<f32>().unwrap();

    let result = calculate(first_number, operator, second_number);

    // Println Macro
    println!("Equation: {:?}", output_equation(first_number, operator, second_number, result));
}

pub fn calculate(first_number: f32, operator: char, second_number: f32) -> f32 {
    /*
    if operator == '+' {
        return first_number + second_number;
    } else if operator == '-' {
        return first_number - second_number;
    } else if operator == '*' {
        return first_number * second_number;
    } else if operator == '/' {
        return first_number / second_number;
    } else {
        return panic!("Invalid Operator used");
    }
    */
    // Using match and implicit Return
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '*' | 'x' => first_number * second_number,
        '/' => first_number / second_number,
        // Else Case
        _ => panic!("Invalid Operator used")
    }
}

pub fn output_equation(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
    // Format Macro
    return format!("{} {} {} = {}", first_number, operator, second_number, result);
}