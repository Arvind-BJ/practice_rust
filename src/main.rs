mod helper_function;
use std::io;

fn main() {
    let mut result: i32 = 0;
    let mut option = String::new();
    helper_function::show_choices();
    
    loop {
        println!("Enter your choice: ");

        let choice = helper_function::get_num();
        match choice {
            1 => {
                //addition
                let num1 = helper_function::get_num();
                let num2 = helper_function::get_num();
                result = num1 + num2;
                println!("Result: {num1} + {num2} = {result}");
            }
            2 => {
                //subtraction
                let num1 = helper_function::get_num();
                let num2 = helper_function::get_num();
                result = num1 - num2;
                println!("Result: {num1} - {num2} = {result}");
            }
            3 => {
                //multiplication
                let num1 = helper_function::get_num();
                let num2 = helper_function::get_num();
                result = num1 * num2;
                println!("Result: {num1} * {num2} = {result}");
            }
            4 => {
                //division
                let num1 = helper_function::get_num();
                let num2 = helper_function::get_num();
                result = num1 / num2;
                println!("Result: {num1} / {num2} = {result}");
            }
            5 => {
                //modulo
                let num1 = helper_function::get_num();
                let num2 = helper_function::get_num();
                result = num1 % num2;
                println!("Result: {num1} % {num2} = {result}");
            }
            _ => {}
        }
        println!("Do you want to continue: (y/N): ");
        option.clear();
        io::stdin()
            .read_line(&mut option)
            .expect("failed to read from stdin");
        let ch = option.trim().parse::<char>().ok().unwrap();
        if ch != 'y' && ch != 'Y' {
            break;
        }
    }
}
