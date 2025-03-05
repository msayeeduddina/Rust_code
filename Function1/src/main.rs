// The `main` function serves as the entry point of the Rust program.
// basic Rust syntax such as function calls, constant usage, block expressions, etc.
fn main() {
    hello_world(); // Calls `tell_height` with an argument of 182 (u32), displaying a height in centimeters.
    tell_height(182); // Calls `human_id` with a string slice "John", age 30 (u32), and height 182.3 (f32),
    human_id("John", 30, 182.3);
    println!("MAX_ATTEMPTS: {}", MAX_ATTEMPTS);
    // Declares an unused variable `_x` (i32) initialized with a block expression. The underscore prefix suppresses unused variable warnings. The block calculates a product of price and quantity and implicitly returns it as the value of `_x`.
    let _x: i32 = {
        let price: i32 = 5; // Local variable for price.
        let qty: i32 = 10; // Local variable for quantity.
        price * qty // Expression (no semicolon) returns the result of multiplication.
    };
    println!("Result is x: {}", _x); // Declares `y` and assigns it the result of the `add` function with arguments 4 and 5.
    let y = add(4, 5);
    println!("Result is y: {}", y); // Directly prints the result of calling `add` with arguments 5 and 6,
    println!("The Add of a and b is {}", add(5, 6));
    let weight = 70.55;
    let height = 1.823;
    let bmi = calculate_bmi(weight, height);
    println!("Calculated BMI is: {:.3}", bmi);
}

// Defines a constant `MAX_ATTEMPTS` with type `u32` and value 5.
// Constants in Rust are immutable, computed at compile time, and must have an explicit type.
// Naming convention uses uppercase with underscores (snake_case).
const MAX_ATTEMPTS: u32 = 5;

// Defines a simple function `hello_world` that takes no parameters and returns no value (implicitly `()`).
fn hello_world() {
    println!("Hello World");
}

// Defines a function `tell_height` that takes a single parameter `height` of type `u32`.
// The function implicitly returns the unit type `()` as it has no explicit return statement.
fn tell_height(height: u32) {
    println!("Hey my height is {} cm.", height);
}

// Defines a function `add` that takes two parameters `a` and `b`, both of type `i32`.
// It returns their sum as an `i32`. The lack of a semicolon in the expression `a + b`
// indicates that this is the return value (Rust's expression-based return mechanism).
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// - `name`: a string slice (`&str`), borrowed immutably.
// - `age`: an unsigned 32-bit integer (`u32`).
// - `height`: a 32-bit floating-point number (`f32`).
// The function implicitly returns `()` as it performs an action without a return value.
fn human_id(name: &str, age: u32, height: f32) {
    println!(
        "Hey this human details are: Name - {}, Age - {}, Height - {}",
        name, age, height
    );
}

fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}
