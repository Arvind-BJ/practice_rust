use std::io;

pub fn get_num() -> i32 {
    let mut input = String::new();

    println!("Enter num: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read from stdin");
    let input = input.trim().parse::<i32>().ok().unwrap();
    input
}

pub fn show_choices() {
    println!("1. addition");
    println!("2. subtraction");
    println!("3. multiplication");
    println!("4. division");
    println!("5. modulo");
}