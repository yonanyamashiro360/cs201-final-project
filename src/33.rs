fn main() {
    // Example code 1: Function definition
    fn print_message(message: &str) {
        println!("{}", message);
    }

    // Example code 2: Main function call
    let result = print_message("Hello, Rust!");
    if let Err(e) = result {
        eprintln!("Error printing message: {}", e);
    }
}
