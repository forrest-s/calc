    use std::io;

    fn main() {
        println!("Welcome to the terminal calculator!");

        loop {
            let operation = &get_operation();
            let numbers = get_numbers();
            let answer = perform_operation(operation, numbers);
            println!("Answer: {}", answer);
        }
    }

    fn get_operation() -> String {
        loop {
            println!("Please enter an operation (+, -, *, /):");
    
            let mut operation = String::new();
            io::stdin()
                .read_line(&mut operation)
                .expect("Failed to read operation");
    
            operation = operation.trim().to_string();
            if operation == "+" || operation == "-" || operation == "*" || operation == "/" {
                return operation;
            }
            println!("Invalid operation entered, please try again");
        }
    }
    



    fn get_numbers() -> (f64, f64) {
        loop {
            println!("Please enter the first number:");

            let mut first_operand = String::new();
            io::stdin()
                .read_line(&mut first_operand)
                .expect("Failed to read first number");

            let first_operand: f64 = match first_operand.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid number entered, please try again");
                    continue;
                }
            };

            println!("Please enter the second number:");

            let mut second_operand = String::new();
            io::stdin()
                .read_line(&mut second_operand)
                .expect("Failed to read second number");

            let second_operand: f64 = match second_operand.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid number entered, please try again");
                    continue;
                }
            };

            return (first_operand, second_operand);
        }
    }

    fn perform_operation(operation: &str, operands: (f64, f64)) -> f64 {
        match operation {
            "+" => operands.0 + operands.1,
            "-" => operands.0 - operands.1,
            "*" => operands.0 * operands.1,
            "/" => operands.0 / operands.1,
            _ => panic!("Invalid operation"),
        }
    }
    
