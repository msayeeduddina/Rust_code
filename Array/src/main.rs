fn main() {
    // Step 1: Declare a mutable array of integers
    // Arrays in Rust have fixed size and type. 'mut' allows modification of elements.
    let mut integer_array = [1, 2, 3, 4, 5];

    // Step 2: Access and print the first element of the array
    // Array indexing starts at 0. Accessing elements uses [index] syntax.
    println!("The first element is {}", integer_array[0]);

    // Step 3: Print the entire array using debug formatting
    // '{:?}' is used for debug printing of collections.
    println!("Integer Array: {:?}", integer_array);

    // Step 4: Modify the third element of the array
    // Arrays are mutable if declared with 'mut'. Index 2 is the third element.
    integer_array[2] = 30;

    // Step 5: Print the updated array
    println!("Updated Integer Array: {:?}", integer_array);

    // Step 6: Print the length of the array
    // '.len()' returns the number of elements in the array.
    println!("Length of Integer Array: {}", integer_array.len());

    // Step 7: Declare an array of string slices with a fixed length
    // Arrays can store string slices (&str) and must specify length and type.
    let string_array: [&str; 3] = ["Apple", "Ball", "Cat"];

    // Step 8: Print the string array
    println!("String Array: {:?}", string_array);

    // Step 9: Pass the string array to a function (by value)
    // Passing by value copies the array. The original is not affected by modifications inside the function.
    modify_string_array_by_value(string_array);

    // Step 10: Print the string array again (should remain unchanged)
    // Confirms that passing by value does not mutate the original array.
    println!("String Array after Value Modification: {:?}", string_array);

    // Step 11: Declare a mutable array of string slices
    let mut mutable_string_array = ["Doll", "Elephant", "Fish"];

    // Step 12: Print the mutable string array
    println!("Mutable String Array: {:?}", mutable_string_array);

    // Step 13: Pass the mutable string array to a function (by mutable reference)
    // Passing by mutable reference (&mut) allows the function to modify the original array.
    modify_string_array_by_reference(&mut mutable_string_array);

    // Step 14: Print the mutable string array again (should reflect changes)
    // Confirms that modifications via mutable reference persist outside the function.
    println!(
        "Mutable String Array after Reference Modification: {:?}",
        mutable_string_array
    );
}

// Function that takes an array of 3 string slices by value and modifies its first element
// The parameter 'mut array' allows modification, but only within the function scope.
fn modify_string_array_by_value(mut array: [&str; 3]) {
    array[0] = "Ant"; // Change the first element
    println!("Modified Array (by Value): {:?}", array); // Print the modified array (inside the function)
                                                        // Changes do not affect the original array in 'main'
}

// Function that takes a mutable reference to an array of 3 string slices
// Modifications affect the original array in the caller's scope.
fn modify_string_array_by_reference(array: &mut [&str; 3]) {
    array[0] = "Deer"; // Change the first element
    println!("Modified Array (by Reference): {:?}", array); // Print the modified array (inside the function)
}
