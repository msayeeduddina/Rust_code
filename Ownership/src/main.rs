fn main() {
    // Call the function get_string() to obtain a string and store it in variable s1.
    // Ownership of the string returned by get_string() is transferred to s1.
    let s1 = get_string();
    // Print the value of s1 to the console.
    println!("This is s1: {}", s1);

    // Create a new string literal "World" and store it in variable s2.
    // s2 owns the string "World".
    let s2 = String::from("World");
    // Call the function send_get_string() with s2 as the argument.
    // Ownership of s2 is transferred to the function send_get_string().
    // The function returns the string, and ownership is then transferred to s3.
    let s3 = send_get_string(s2);
    // Print the value of s3 to the console.
    println!("This is s3: {}", s3);

    // Create a new string literal "What's up" and store it in variable s4_var.
    // s4_var owns the string "What's up".
    let s4_var = String::from("What's up");
    // Call the function destructure_item() with s4_var as the argument.
    // Ownership of s4_var is transferred to the function destructure_item().
    // The function returns a tuple containing the original string and its length.
    // Ownership of the string is transferred back to s4_str.
    let (s4_str, s4_len) = destructure_item(s4_var);
    // Print the value of s4_str and its length.
    println!("This is s4: {} having length {}", s4_str, s4_len);

    // Create a new string literal "Hii" and store it in variable s5_var.
    // s5_var owns the string "Hii".
    let s5_var = String::from("Hii");
    // Call the function clone_item() with a clone of s5_var as the argument.
    // The clone of s5_var is created using the clone() method, which creates a new string with the same content.
    // Ownership of the cloned string is transferred to the function clone_item().
    // The function returns the length of the cloned string.
    let s5_len: usize = clone_item(s5_var.clone());
    // Print the length of the cloned string.
    println!("This is s6: {}", s5_len);

    // Create a new string literal "Dude" and store it in variable s6_var.
    // s6_var owns the string "Dude".
    let s6_var = String::from("Dude");
    // Call the function borrow_item() with a reference to s6_var as the argument.
    // The reference to s6_var is created using the & operator.
    // The function borrows the string and returns its length.
    let s6_len = borrow_item(&s6_var);
    // Print the value of s6_var and its length.
    println!("This is s6: {} having length {}", s6_var, s6_len);
}

// Define a function get_string() that returns a String.
// This function creates a new string and transfers ownership of it to the caller.
fn get_string() -> String {
    // Create a new string literal "Hello" and store it in variable new_string.
    // new_string owns the string "Hello".
    let new_string: String = String::from("Hello");
    // Return the new_string.
    // Ownership of new_string is transferred to the caller of get_string().
    return new_string;
}

// Define a function send_get_string() that takes a String as an argument and returns it.
// This function takes ownership of the string passed to it and then transfers ownership back to the caller.
fn send_get_string(item: String) -> String {
    // Return the item passed as an argument.
    // Ownership of item is transferred to the caller of send_get_string().
    return item;
}

// Define a function destructure_item() that takes a String as an argument and returns a tuple containing the string and its length.
// This function takes ownership of the string passed to it and then transfers ownership of the string back to the caller.
fn destructure_item(item: String) -> (String, usize) {
    // Calculate the length of the string.
    let l = item.len();
    // Return a tuple containing the original string and its length.
    // Ownership of the string is transferred back to the caller.
    return (item, l);
}

// Define a function clone_item() that takes a String as an argument and returns its length.
// This function takes ownership of the string passed to it.
fn clone_item(item: String) -> usize {
    // Return the length of the string.
    // The string is dropped when the function returns, as it is no longer needed.
    return item.len();
}

// Define a function borrow_item() that takes a reference to a String as an argument and returns its length.
// This function borrows the string and does not take ownership.
fn borrow_item(item: &String) -> usize {
    // Return the length of the string.
    // The reference is dropped when the function returns, and the original string remains unchanged.
    return item.len();
}
