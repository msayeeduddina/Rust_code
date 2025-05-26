// Import the standard input/output library
use std::io;

fn main() {
    // Create a mutable String to store user input
    let mut input = String::new();
    
    // Prompt the user for input
    println!("Enter an input: ");
    
    // Read a line from standard input and store it in 'input'
    // The expect method handles any potential errors during input
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    // Print the input received from the user
    println!("You entered: {}", input);
}
