fn main() {
    let mut mystr = String::from("Hello");
    // Create a mutable reference to `mystr`.
    let w1 = &mut mystr;
    // Modify the string through the mutable reference.
    w1.push_str(" World");
    // Print the modified string using the mutable reference.
    println!("w1: {}", w1);
    // Print the original string, which is now modified.
    println!("mystr: {}", mystr);

    // Attempt to create another mutable reference to `mystr`.
    // This will cause a compile-time error because Rust's borrowing rules
    // prevent multiple mutable references to the same data at the same time.
    // let w2 = &mut mystr; // Error: cannot borrow `mystr` as mutable more than once at a time

    // To fix this, we need to ensure that the first mutable reference (`w1`) is no longer in scope
    // before creating the second mutable reference (`w2`).
    // One way to do this is to use a block to limit the scope of `w1`.
    {
        let w1 = &mut mystr;
        w1.push_str(" World");
        println!("w1: {}", w1);
    } // `w1` goes out of scope here

    // Now it's safe to create another mutable reference to `mystr`.
    let w2 = &mut mystr;
    w2.push_str(" Code");
    println!("w2: {}", w2);
    println!("mystr: {}", mystr);

    // Attempt to create an immutable reference to `mystr` after a mutable reference.
    // This will cause a compile-time error because Rust's borrowing rules
    // prevent creating an immutable reference while a mutable reference exists.
    // let r2 = &mystr; // Error: cannot borrow `mystr` as immutable because it is also borrowed as mutable

    // To fix this, we need to ensure that the mutable reference (`w2`) is no longer in scope
    // before creating the immutable reference (`r2`).
    // One way to do this is to use a block to limit the scope of `w2`.
    {
        let w2 = &mut mystr;
        w2.push_str(" Code");
        println!("w2: {}", w2);
    } // `w2` goes out of scope here

    // Now it's safe to create an immutable reference to `mystr`.
    let r2 = &mystr;
    println!("r2: {}", r2);
}
