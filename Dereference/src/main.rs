fn main() {
    let mut x = 5; // Declare a mutable variable x and initialize it to 5
    x = x + 1; // Increment x by 1 (x is now 6)
    println!("address={:p}", &x); // Print the memory address of x
    println!("The value of x is: {}", x); // Print the current value of x

    let y = &mut x; // Create a mutable reference to x (y can now modify x)
    println!("The value of y is: {}", y); // Print the value y points to (dereferences automatically in println!)

    *y = *y + 1; // Dereference y, increment the value it points to (x is now 7)
    println!("The value of y is: {}", y); // Print the updated value via y
}
