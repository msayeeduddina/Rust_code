fn main() {
    // Create a mutable array (though we never mutate it)
    let mut arr_it = [1, 2, 3, 4, 5];

    // Iterate over immutable references to each element
    // .iter() yields &i32; val is a reference to the current item
    for val in arr_it.iter() {
        println!("The value is: {}", val);
    }

    // Obtain an iterator over the slice and pull the first item
    // .next() returns an Option<&i32>: Some(&1) here
    println!("1st : {:?}", arr_it.iter().next());
}