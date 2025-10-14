fn main() {
    stack_fn();
    heap_fn();
    update_string();
}

/// Demonstrates stack allocation
/// Stack: Fast, fixed-size, automatic cleanup
/// Primitive types like i32 are stored on the stack
fn stack_fn() {
    // All these values live on the stack
    // Stack memory is automatically freed when function ends
    let a = 5;
    let b = 6;
    let c = a + b;
    println!("Stack function c = {}", c);
}

/// Demonstrates heap allocation
/// Heap: Flexible size, manual management (Rust handles via ownership)
/// String data is stored on the heap, pointer on the stack
fn heap_fn() {
    // String data lives on the heap
    // s1 and s2 are pointers (on stack) to heap data
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    // format! creates a new String on the heap
    // s1 and s2 are borrowed (not moved) by format! macro
    let s12 = format!("{} {}", s1, s2);
    println!("Heap function s12 = {}", s12);
}

/// Demonstrates mutable String operations on the heap
/// String can grow/shrink dynamically (unlike stack data)
/// Shows capacity, length, and pointer address changes
fn update_string() {
    // mut allows modification of the String
    // String capacity may grow when we push more data
    let mut s3 = String::from("Hello");
    println!("Before update s3 = {}", s3);
    // String internals on the stack:
    // - capacity(): allocated space on heap (bytes)
    // - len(): actual used space (bytes)
    // - as_ptr(): memory address of heap data
    println!("s3 capacity = {}, s3 length = {}, s3 pointer = {:p}", s3.capacity(), s3.len(), s3.as_ptr());
    // push_str appends to existing String
    // May reallocate if capacity is exceeded (pointer changes)
    s3.push_str(", World!");
    println!("After update s3 = {}", s3);
    // Check if reallocation occurred
    // If pointer changed, String was moved to new heap location
    // Capacity typically doubles when reallocation happens
    println!("s3 capacity = {}, s3 length = {}, s3 pointer = {:p}", s3.capacity(), s3.len(), s3.as_ptr());
}