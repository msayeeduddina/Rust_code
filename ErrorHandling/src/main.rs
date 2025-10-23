use std::fs; // Import the 'fs' (filesystem) module from the standard library
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;

/// Generic Point struct with two type parameters.
/// This demonstrates **Generics** in Rust, allowing the struct to hold different data types.
/// A and B can be any type (not necessarily the same).
/// 
/// # Type Parameters
/// * `A` - Type for the x coordinate
/// * `B` - Type for the y and z coordinates (must be the same type)
/// 
/// # Fields
/// * `x` - First coordinate of type A
/// * `y` - Second coordinate of type B
/// * `z` - Third coordinate of type B
struct Point<A, B> {
    x: A,
    y: B,
    z: B,
}

/// Demonstrates how to instantiate the generic Point struct with different concrete types.
/// The compiler infers the concrete types (e.g., i32, &str, f64) based on the initialization values.
fn call_point() {
    // Point<i32, &str> - x is integer (i32), y and z are string slices (&str)
    let interger_point = Point { 
        x: 5, // i32
        y: "10", // &str (string slice)
        z: "15.1" // &str (string slice)
    };
    // Point<f64, f64> - all coordinates are floating-point numbers (f64)
    let float_point = Point { 
        x: 1.0, 
        y: 2.0, 
        z: 3.0 
    };
    // Point<&str, f64> - x is string slice (&str), y and z are floats (f64)
    let mix_point = Point { 
        x: "5", 
        y: 10.0, 
        z: 15.0 
    };
    // Accessing and printing the struct fields
    println!("interger_point.x = {}", interger_point.x);
    println!("float_point.y = {}", float_point.y);
    println!("mix_point.z = {}", mix_point.z);
}

/// Attempts to read a file, handling potential I/O errors gracefully.
/// Uses the **match** expression to destructure the **Result** enum (which `fs::read_to_string` returns).
/// If an error occurs (e.g., file not found), the program prints the error message and continues execution.
fn check_file_safe(){
    // fs::read_to_string returns a standard library Result<String, io::Error>
    let res = fs::read_to_string("example.txt");
    
    // Match handles both Ok (success) and Err (failure) cases
    match res{
        Ok(content) => println!("File content: {}", content), // Success: prints content
        Err(e) => println!("Error reading file: {}", e), // Error: prints error details
    }
    // Execution always continues here, demonstrating non-panicking error handling
    println!("Execution continues after error handling.");
}

/// Attempts to read a file, using **unwrap()** for immediate error extraction.
/// **unwrap()** will **panic** (crash the program) if the result is an Err.
/// This method is discouraged for general error handling.
fn check_file_unsafe(){
    // unwrap() extracts the value from Ok, or causes a program crash (panic) if it's Err
    let content = fs::read_to_string("example.txt").unwrap(); 
    println!("File content: {}", content);
    // This line is only reached if the file read succeeds without panicking
    println!("Execution continues after error handling.");
}

// ---------------------- New Code Documentation ----------------------

/// Custom generic enumeration similar to the standard library's **Option<T>**.
/// Used to represent the possible presence or absence of a value.
/// 
/// # Type Parameters
/// * `T` - The type of the value held in the `Some` variant.
pub enum MyOption<T> {
    // Represents no value present
    None,
    // Represents a value of type T present
    Some(T),
}

/// Searches for the first occurrence of the character 'o' in the given string.
/// 
/// **Note**: The function signature shows it takes ownership of the `String` (`s`).
/// 
/// # Arguments
/// * `s` - The String to search within (ownership is moved into this function).
/// 
/// # Returns
/// An `MyOption<usize>`: 
/// * `MyOption::Some(index)` if 'o' is found, where index is the byte offset.
/// * `MyOption::None` if 'o' is not found.
fn find_first_a(s: String) -> MyOption<usize> {
    // Iterate over characters and their byte indices using enumerate()
    for (index, character) in s.chars().enumerate() {
        // Check if the current character is 'o'
        if character == 'o' {
            // Return Some() variant with the index if found
            return MyOption::Some(index);
        }
    }
    // If the loop completes without finding 'o', return the None variant
    MyOption::None
}

/// Calls `find_first_a` and handles the returned `MyOption` using a **match** expression.
fn check_find_first_a() {
    // Create a String (owned data on the heap)
    let my_string = String::from("hello world"); 
    // Call the search function. Ownership of my_string is moved here.
    let res = find_first_a(my_string);
    // Match the custom enum result
    match res {
        // Found case: index is extracted from the Some variant
        MyOption::Some(index) => println!("Found 'o' at index: {}", index), 
        // Not found case: None variant
        MyOption::None => println!("'o' not found in the string"),
    }
}

fn extra_error_handling_demo() {
    // ----- READING & WRITING TO FILES & ERROR HANDLING -----
    // Rust doesn't have exceptions like other languages. It handles
    // recoverable errors with Result and the panic! macro for
    // unrecoverable errors
    // When the panic! macro executes your program prints an error
    // memory is cleaned up and the program quits
    // panic!("Terrible Error");
    // Accessing an index that doesn't exist calls panic
    // let lil_arr = [1,2];
    // println!("{}", lil_arr[10]);
    // File to create
    let path = "lines.txt";
    // Result has 2 variants: Ok and Err
    // enum Result<T, E> {
    //   Ok(T),
    //   Err(E),
    // }
    // T is the type of the success value, E is the error type
    // Create the file; match on the Result to handle success or failure
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file, // on success, bind the file handle
            Err(error) => { // on failure, panic with a custom message
                panic!("Problem creating file: {:?}", error);
            }
    };
    // Write to the file; expect() panics with the given message if write! fails
    write!(output, "Just some\nRandom Words").expect("Failed to write to file");
    // Open the file for reading; unwrap() yields the file or panics on error
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    // Iterate over lines; each line is a Result<String, io::Error>
    for line in buffered.lines() {
        // unwrap() each line or panic if reading fails
        println!("{}", line.unwrap());
    }
    // Demonstrate fine-grained error handling: retry creation if file not found
    let output2 = File::create("rand.txt");
    let output2 = match output2 {
        Ok(file) => file, // success path
            Err(error) => match error.kind() { // inspect the error kind
                ErrorKind::NotFound => // file didn't exist; try to create it
                    match File::create("rand.txt") {
                        Ok(fc) => fc, // creation succeeded
                            Err(e) => panic!("Can't create file: {:?}", e),
                    },
                    _other_error => panic!("Problem opening file: {:?}", error), // any other error
            },
    };
}



/// Main entry point of the program.
fn main() {
    // Call function demonstrating generic struct usage
    call_point();
    // Call function demonstrating safe file error handling (Result/match)
    check_file_safe();
    // Call function demonstrating custom Option enum usage and handling
    check_find_first_a();
    // Calling this function would crash the program if "example.txt" is missing
    // check_file_unsafe();
    extra_error_handling_demo(); 
}