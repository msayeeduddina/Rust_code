/// Main entry point of the program
/// Demonstrates Rust's type system and default numeric types
fn main() {
    // Signed 32-bit integer (can be negative)
    // Default type for integers if not specified
    let x: i32 = -5;
    
    // Unsigned 32-bit integer (only positive values)
    // Without explicit type, literals default to i32
    let y: u32 = 1000;
    
    // 32-bit floating point number
    // Without explicit type, float literals default to f64
    let z: f32 = 1000.001;
    
    println!("x is {}, y is {}, z is {}", x, y, z);
    check_overflow_underflow();
}

/// Demonstrates integer overflow behavior in Rust
/// 
/// In debug mode: panics on overflow
/// In release mode: wraps around (overflow behavior)
/// 
/// i8 range: -128 to 127
/// Starting at 100, adding 100 repeatedly causes overflow
fn check_overflow_underflow() {
    // mut keyword allows variable to be modified
    let mut x: i8 = 100;
    
    // Loop from 0 to 999 (1000 iterations)
    for i in 0..1000 {
        x = x + 100;  // This will overflow after first iteration
    }
    
    // Result depends on compilation mode:
    // Debug: program panics before reaching here
    // Release (--release flag): wraps around, final value unpredictable
    println!("check_X is {}", x);
}