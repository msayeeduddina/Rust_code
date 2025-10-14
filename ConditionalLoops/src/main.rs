fn main() {
    // String - heap-allocated, owned data
    let sentence = String::from("Hello world");
    
    // Pass ownership of sentence to function
    // After this call, sentence is moved and cannot be used again
    let first_word = get_first_word(sentence);
    
    // Range loop: 0..10 means 0 to 9 (exclusive end)
    for i in 0..10 {
        println!("hello world loop {}", i);
    }
    
    println!("The first word is {}", first_word);

    // Function call with two arguments
    // Values are copied (i32 implements Copy trait)
    let a = 5;
    let b = 6;
    let sum = do_sum(a, b);
    
    // Variables a and b are still usable after function call (Copy trait)
    println!("The sum of {} and {} is {}", a, b, sum);
}

/// Extracts the first word from a sentence (up to first space)
/// Takes ownership of the String (moves it)
/// Returns a new String containing the first word
fn get_first_word(sentence: String) -> String {
    // mut allows the variable to be modified
    let mut ans = String::from("");
    
    // Iterate over each character in the string
    for char in sentence.chars() {
        // push_str adds a string slice to the String
        // char.to_string() converts char to String
        ans.push_str(char.to_string().as_str());
        
        // Break loop when space is encountered
        if char == ' ' {
            break;
        }
    }
    
    // return keyword is optional for last expression
    return ans;
}

/// Adds two 32-bit integers and returns the result
/// Parameters are copied (not moved) because i32 implements Copy trait
/// 
/// # Arguments
/// * `a` - First number to add
/// * `b` - Second number to add
/// 
/// # Returns
/// Sum of a and b as i32
fn do_sum(a: i32, b: i32) -> i32 {
    return a + b;
    // More idiomatic: a + b (without return keyword)
}