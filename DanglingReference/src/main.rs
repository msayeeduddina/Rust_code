// This function attempts to return a reference to a String
fn create_string_ref() -> &String {
    // A new String is created on the heap and bound to 's'
    let s = String::from("hello");
    println!("{}", s); // The string is printed to the console

    // Here, we attempt to return a reference to 's'
    // However, 's' is a local variable and will be dropped when the function ends
    // Returning a reference to it will cause a dangling reference!
    return &s;
}

fn main() {
    // This line tries to get a reference to a String from create_string_ref()
    // But the reference points to memory that is already freed (dangling reference)
    let reference_to_nothing = create_string_ref();
    // Using 'reference_to_nothing' here would be undefined behavior
}
