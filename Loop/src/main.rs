fn main() {
    // Function to check if a number is even
    fn is_even(num: i8) -> bool {
        // Check if the number is divisible by 2
        if num % 2 == 0 {
            return true; // Return true if the number is even
        } else {
            return false; // Return false if the number is odd
        }
    }

    // Declare a variable to hold the number to check
    let number = 5;

    // Match statement to print messages based on the value of 'number'
    match number {
        1 | 2 => println!("Number is One or Two"), // Handle cases for 1 and 2
        3 | 4 => println!("Number is Three or Four"), // Handle cases for 3 and 4
        5 => println!("Number is Five"), // Handle case for 5
        _ => println!("Number is not in range"), // Handle all other cases
    }

    // Another match statement to check if the number is even or odd
    match number {
        x if is_even(x) => println!("Number is even"), // Print if the number is even
        x if !is_even(x) => println!("Number is odd"), // Print if the number is odd
        _ => println!("Number is not in range"), // Handle all other cases
    }
}
