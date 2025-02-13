fn main() {
    // **Integer Types**
    let signed_int: i32 = -10; // Signed 32-bit integer
    let unsigned_int: u32 = 10; // Unsigned 32-bit integer
    let large_signed: i64 = 9_223_372_036_854_775_807; // Large signed integer
    let small_unsigned: u8 = 255; // Maximum value for an 8-bit unsigned integer

    // **Floating-Point Numbers**
    let float_num: f32 = 3.14; // 32-bit floating-point number
    let double_num: f64 = 2.71828; // 64-bit floating-point number

    // **Boolean Types**
    let is_active: bool = true; // Boolean value
    let is_admin: bool = false; // Another boolean value

    // **Character Types**
    let letter: char = 'A'; // Single character
    let emoji: char = '😊'; // Unicode character

    // **Printing the values**
    println!("Signed Integer: {}", signed_int);
    println!("Unsigned Integer: {}", unsigned_int);
    println!("Large Signed Integer: {}", large_signed);
    println!("Small Unsigned Integer: {}", small_unsigned);
    println!("Floating Point Number (f32): {}", float_num);
    println!("Floating Point Number (f64): {}", double_num);
    println!("Boolean (is_active): {}", is_active);
    println!("Boolean (is_admin): {}", is_admin);
    println!("Character: {}", letter);
    println!("Emoji: {}", emoji);
}
