// Compiler directive - suppresses warnings for unused code
#![allow(unused)]

// Import statements - bring external modules into scope
use std::io; // Standard input/output module
use std::io::{
    Write,
    BufReader,
    BufRead,
    ErrorKind
}; // I/O traits and types
use std::fs::File; // File handling
use std::cmp::Ordering; // Comparison ordering enum
use std::collections::HashMap; // Hash map collection type

/// Creates and demonstrates basic HashMap operations with superhero data.
///
/// # Description
/// This function initializes a HashMap with superhero aliases as keys and their
/// real names as values. It demonstrates insertion, iteration, length checking,
/// key existence verification, and value retrieval with proper error handling.
///
/// # Type Inference
/// The HashMap type `HashMap<&str, &str>` is inferred from the first insert operation.
///
/// # Memory
/// - HashMap is allocated on the heap with default initial capacity
/// - Automatically deallocated when function scope ends (RAII)
///
/// # Time Complexity
/// - Insertions: O(1) average case
/// - Iteration: O(n) where n is the number of entries
/// - `contains_key()`: O(1) average case
/// - `get()`: O(1) average case
///
/// # Output
/// Prints to stdout:
/// - All key-value pairs (order not guaranteed)
/// - Total number of entries
/// - Batman's real name if found
///
/// # Examples
/// ```
/// create_hashmap();
/// // Output (order may vary):
/// // Superman = Clark Kent
/// // Batman = Bruce Wayne
/// // Flash = Barry Allen
/// // Length: 3
/// // Found Batman: Bruce Wayne
/// ```
fn create_hashmap() {
    // HashMap::new() - Creates empty hash map with default capacity (0)
    // 'mut' keyword - makes the binding mutable (allows insert operations)
    // Type: HashMap<&str, &str> - keys and values are string slices
    let mut heroes = HashMap::new();

    // insert() method signature: pub fn insert(&mut self, k: K, v: V) -> Option<V>
    // Returns: Some(old_value) if key existed, None if new key
    // Key: &str (string slice), Value: &str (string slice)
    // These operations may trigger reallocation if capacity is exceeded
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("Flash", "Barry Allen");

    // iter() method signature: pub fn iter(&self) -> Iter<'_, K, V>
    // Returns: Iterator yielding (&K, &V) tuples (immutably borrowed references)
    // (k, v) - tuple destructuring in for loop pattern matching
    // Order: Not guaranteed - depends on internal hash bucket ordering
    for (k, v) in heroes.iter() {
        // println! macro - formats and prints to stdout with newline
        // {} - placeholder for Display trait formatting
        println!("{} = {}", k, v);
    }

    // len() method signature: pub fn len(&self) -> usize
    // Returns: Number of key-value pairs in the HashMap
    println!("Length: {}", heroes.len());

    // contains_key() method signature: pub fn contains_key<Q>(&self, k: &Q) -> bool
    // Returns: true if key exists, false otherwise
    // Time Complexity: O(1) average case
    if heroes.contains_key("Batman") {
        // get() method signature: pub fn get<Q>(&self, k: &Q) -> Option<&V>
        // Returns: Some(&value) if key exists, None if key not found
        // Returns a reference to avoid moving/copying the value
        let the_batman = heroes.get("Batman");

        // match expression - pattern matching on Option enum
        // Exhaustive: must handle all variants (Some, None)
        match the_batman {
            Some(x) => println!("Found Batman: {}", x), // x is &&str (reference to the value)
                None => println!("Batman not found"), // This branch is unreachable due to contains_key check
        }
    }
}

/// Program entry point.
///
/// # Description
/// The main function is the entry point for the executable. It is automatically
/// called by the Rust runtime when the program starts.
///
/// # Returns
/// Returns `()` (unit type) indicating successful completion. The program exits
/// with status code 0 on normal termination.
///
/// # Execution Flow
/// 1. Calls `create_hashmap()` to demonstrate HashMap operations
/// 2. Returns control to the Rust runtime
/// 3. Runtime performs cleanup and exits
fn main() {
    // Function call - executes create_hashmap()
    // The function borrows nothing and returns nothing
    create_hashmap();
}

// =============================================================================
// KEY TECHNICAL ASPECTS:
// =============================================================================
//
// 1. OWNERSHIP & BORROWING:
//    - HashMap takes ownership of values (for non-Copy types)
//    - &str is Copy (since it's just a pointer + length), so values are copied
//    - iter() returns borrowed references, not owned values
//    - get() returns Option<&V> (borrowed reference) to avoid moving
//
// 2. TYPE SYSTEM:
//    - Static typing with type inference
//    - HashMap<K, V> is generic over key (K) and value (V) types
//    - Type constraints: K must implement Eq + Hash
//
// 3. MEMORY MANAGEMENT:
//    - No garbage collector - automatic memory management via ownership
//    - HashMap dropped automatically when out of scope (RAII pattern)
//    - Stack: Function local variables and references
//    - Heap: HashMap internal buffer (Vec<Bucket<K, V>>)
//
// 4. TIME COMPLEXITY:
//    - insert(): Average O(1), Worst O(n) during rehashing
//    - get(): Average O(1), Worst O(n) with poor hash function
//    - contains_key(): Average O(1), Worst O(n)
//    - iter(): O(n) to iterate all elements
//    - len(): O(1) - stored as field
//
// 5. SPACE COMPLEXITY:
//    - O(n) where n is the number of key-value pairs
//    - Additional overhead for hash buckets (load factor typically 0.75)
//
// 6. TRAITS REQUIRED:
//    - Keys (K) must implement: Eq (equality comparison) + Hash (hashing function)
//    - Values (V) have no trait requirements
//    - &str implements both Eq and Hash (via str)
//
// 7. HASH COLLISION HANDLING:
//    - Uses Robin Hood hashing (as of Rust 1.36+)
//    - Provides better worst-case performance than chaining
//
// 8. THREAD SAFETY:
//    - HashMap is NOT thread-safe (not Send/Sync by default with RefCell)
//    - Use Arc<Mutex<HashMap>> or DashMap for concurrent access
//
// 9. UNUSED IMPORTS:
//    - This code imports io, Write, BufReader, BufRead, ErrorKind, File, and
//      Ordering but doesn't use them (hence #![allow(unused)] directive)
//
// =============================================================================