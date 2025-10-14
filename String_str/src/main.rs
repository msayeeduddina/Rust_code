fn main() {
    // Create a mutable String from a string literal
    // String::from allocates a growable, heap-allocated string.
    // 'mut' allows us to modify the String after creation.
    let mut greeting = String::from("Hello");
    println!("(String) greeting: {}", greeting);

    // Append to the String
    // push_str appends a string slice to the existing String.
    greeting.push_str(" World");
    println!("(String) appending greeting: {}", greeting);

    // String slice (&str) from a string literal
    // String literals are of type &str, which are immutable references to string data.
    let simple_greeting: &str = "Hi"; // Equivalent to: let simple_greeting = "Hi";
    println!("(&str) simple_greeting: {}", simple_greeting);

    // Array of string slices (&str)
    // Arrays in Rust have a fixed size and type.
    let arr_str: [&str; 3] = ["Hello", "World", "Hi"];

    // Pass the array to a function by value (the array is copied)
    read_arr_str(arr_str);

    // Print the original array to show it remains unchanged
    println!("arr_str: {:?}", arr_str);

    // Demonstrate character access in the String
    check_char();
}

// Function that takes an array of 3 string slices by value
// The array is copied into the function (since &str is Copy), so the original is not affected.
fn read_arr_str(arr_str_param: [&str; 3]) {
    println!("arr_str_param: {:?}", arr_str_param);
}

/// Demonstrates character access in Rust strings
/// Shows Option type and unwrap() method
fn check_char() {
    // String type - heap-allocated, growable UTF-8 string
    // String::from() creates a String from a string literal
    let greeting = String::from("Hello World");
    // chars() returns an iterator over characters
    // nth(1) gets the character at index 1 (0-indexed)
    // Returns Option<char> - Some(char) if exists, None if out of bounds
    let char1 = greeting.chars().nth(1);
    // unwrap() extracts value from Option
    // Panics if the value is None (use carefully!)
    print!("char is {}", char1.unwrap());
}