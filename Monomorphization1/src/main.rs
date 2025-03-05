// Allows suppression of unused variable and warning diagnostics for this file.
#![allow(unused, warnings)]

// Generic struct `Point` with type parameter `T`, representing a 2D point with `x` and `y` coordinates of type `T`.
struct Point<T> {
    x: T,
    y: T,
}

// Concrete struct `Point_u32` for 2D points with unsigned 32-bit integer coordinates.
struct Point_u32 {
    x: u32,
    y: u32,
}

// Concrete struct `Point_i32` for 2D points with signed 32-bit integer coordinates.
struct Point_i32 {
    x: i32,
    y: i32,
}

// Generic function `get_x` takes a `Point<T>` and returns its `x` coordinate of type `T`.
fn get_x<T>(p: Point<T>) -> T {
    p.x
}

// Returns the `x` coordinate from a `Point_u32` as a `u32`.
fn get_x_u32(p: Point_u32) -> u32 {
    p.x
}

// Returns the `x` coordinate from a `Point_i32` as an `i32`.
fn get_x_i32(p: Point_i32) -> i32 {
    p.x
}

// Entry point demonstrating monomorphization with generic and concrete structs.
fn main() {
    let p0: Point<u32> = Point { x: 10, y: 0 }; // `Point<u32>` instance.
    let p1: Point<i32> = Point { x: -5, y: 0 }; // `Point<i32>` instance.

    println!("x coordinate of p0: {}", get_x(p0)); // Calls monomorphized `get_x` for `u32`.
    println!("x coordinate of p1: {}", get_x(p1)); // Calls monomorphized `get_x` for `i32`.

    let p0 = Point_u32 { x: 20, y: 0 }; // `Point_u32` instance.
    let p1 = Point_i32 { x: -10, y: 0 }; // `Point_i32` instance.

    println!("x coordinate of p0 (u32): {}", get_x_u32(p0)); // Calls concrete `get_x_u32`.
    println!("x coordinate of p1 (i32): {}", get_x_i32(p1)); // Calls concrete `get_x_i32`.
}

/*
Monomorphization:- is a compilation strategy in languages like Rust where the compiler transforms generic code into specific, concrete implementations by substituting type parameters with actual types at compile time.
For instance, a generic `struct Point<T>` used with `u32` and `i32` results in the compiler generating `Point<u32>` and `Point<i32>`, while a generic function like `fn get_x<T>(p: Point<T>) -> T` becomes separate functions like `get_x_u32` and `get_x_i32`, each tailored to the specific type.
This process ensures type safety and eliminates runtime type resolution, producing optimized, type-specific code.
The trade-off of monomorphization is increased compilation time and binary size due to the duplication of code for each type used, but it delivers significant performance benefits by avoiding runtime overhead, such as dynamic dispatch or type checking.
In Rust, this technique allows developers to write flexible, reusable generic code while maintaining the efficiency of statically typed, non-generic implementations, striking a balance between versatility and performance.
*/
