fn main() {
    // Define an array of integers with a fixed size of 5
    let numbers: [i32; 5] = [11, 22, 33, 44, 55]; // [i32; 5]: Defines an array of i32 integers with a size of 5
    println!("Numbers Array: {:?}", numbers); // {:?}: Formatting specifier for debug printing

    // Define an array of string slices with a fixed size of 3
    let fruits: [&str; 3] = ["Apple", "Banana", "Cherry"]; // [&str; 3]: Defines an array of string slices with a size of 3
    println!("Fruits Array:{:?}", fruits);
    println!("Fruits Array at 1st Element : {}", fruits[0]); // [0]: Accesses the element at index 0 of the array
    println!("Fruits Array at 2nd Element : {}", fruits[1]); // [1]: Accesses the element at index 1 of the array
    println!("Fruits Array at 3rd Element : {}", fruits[2]); // [2]: Accesses the element at index 2 of the array

    // Define a tuple containing a String, an i32, and a bool
    let person_info: (String, i32, bool) = ("Alice".to_string(), 123, true); // : (String, i32, bool) = Defines a tuple type with a String, an i32, and a bool; .to_string(): Converts a string literal to a String
    println!("Person Info Tuple: {:?}", person_info);
    println!("Person Name: {}", person_info.0); // person_info.0: Accesses the first element of the tuple

    // Define a mixed tuple containing various types
    let mixed_tuple = ("Bob", 456, true, [789, 1011, 1213]);
    println!("Mixed Tuple: {:?}", mixed_tuple);

    // Define a slice of integers
    let number_slice: &[i32] = &[1, 2, 3, 4]; // : &[i32] = Defines a slice of i32 integers
    println!("Number Slice: {:?}", number_slice);

    // Define a slice of string slices (animal names)
    let animal_slice: &[&str] = &["Ant", "Cat", "Elephant"];
    println!("Animal Slice: {:?}", animal_slice);

    // Define a slice of references to Strings (fish names)
    let fish_slice: &[&String] = &[
        &"Dolphin".to_string(),
        &"Whale".to_string(),
        &"Shark Group".to_string(),
    ]; // : &[&String] = Defines a slice of references to String objects; .to_string(): Converts a string literal to a String; &: Creates a reference
    println!("Fish Slice: {:?}", fish_slice);

    // Demonstrating Strings vs String Slices
    let mut greeting_from_steve: String = String::from("Hello, "); // : String = String::from("..."): Creates a new String from a string literal
    greeting_from_steve.push_str("World");
    println!("Steve says: {}", greeting_from_steve); // {}: Formatting specifier for string printing

    let greeting_from_smith: String = String::from("Hello World");
    let greeting_from_iris: &str = &greeting_from_smith; // &str = &: Creates a reference to a string slice
    let simple_greeting_from_iris: &str = &greeting_from_smith[0..5]; // [0..5]: Creates a slice of a string from index 0 to 5 (exclusive)

    println!("Iris says: {}", greeting_from_iris);
    println!("Iris simply says: {}", simple_greeting_from_iris);
}
