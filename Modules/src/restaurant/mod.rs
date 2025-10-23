// `mod` keyword defines a module - a namespace for organizing code.
// Modules provide encapsulation and control visibility of items.
// `pizza_order` module contains all pizza-related functionality.
mod pizza_order {

    // `pub` keyword makes the struct visible outside the module.
    // Without `pub`, Pizza would only be accessible within pizza_order module.
    // `struct` defines a custom data type representing a pizza.
    pub struct Pizza {
        // `pub` on fields makes them directly accessible from outside the struct.
        // `String` is a heap-allocated, growable, UTF-8 encoded string type.
        // Each field represents a pizza component.
        pub dough: String,
        pub cheese: String,
        pub topping: String,
    }

    // `impl` block defines methods associated with the Pizza struct.
    // All functions here are associated with Pizza type.
    impl Pizza {
        // `pub fn` makes this function publicly accessible.
        // `lunch` is an associated function (no self parameter) - a constructor.
        // `topping` parameter (note: typo in original "tooping") specifies the topping.
        // `-> Pizza` specifies the return type.
        pub fn lunch(topping: String) -> Pizza {
            // Creates and returns a new Pizza instance.
            // `String::from()` converts string literals (&str) to owned String.
            // Field init: when variable name matches field name, shorthand can be used.
            Pizza {
                // `String::from("cross")` creates a String from a string literal.
                // String literals (&str) are immutable and stored in program binary.
                dough: String::from("regular crust"),
                cheese: String::from("mozzarella"),
                // `topping` uses the parameter value passed to the function.
                // Could also be written as: topping: topping (long form).
                topping,
            }
        }
    }

    // `pub mod` creates a public submodule within pizza_order.
    // Nested modules help organize related functionality hierarchically.
    // This module handles customer service operations.
    pub mod help_customer {

        // Private function (no `pub` keyword) - only accessible within this module.
        // `fn` defines a function with no parameters and no return value (returns unit `()`).
        // Functions without `pub` are module-private by default.
        fn seat_at_table() {
            // `println!` macro prints formatted text with a newline to stdout.
            println!("Customer seated at table.");
        }

        // `pub fn` makes this function callable from outside the module.
        // This is the main entry point for the ordering process.
        // No parameters, returns unit type `()`.
        pub fn take_order() {
            // Calls the private function within the same module.
            // No path needed - same module scope.
            seat_at_table();
            // `let` declares an immutable variable binding.
            // `super::Pizza` uses the `super` keyword to access parent module.
            // `super` refers to pizza_order module (one level up).
            // `::` is the path separator for navigating module hierarchy.
            let customer_order: super::Pizza = super::Pizza::lunch(String::from("veggies"));
            // Calls private function, passing the Pizza instance.
            // Ownership of customer_order moves to serve_customer.
            serve_customer(customer_order);
        }

        // Private function that takes ownership of a Pizza instance.
        // `cust_pizza: super::Pizza` parameter takes ownership (not borrowed).
        // When function ends, cust_pizza is dropped (memory cleaned up).
        fn serve_customer(cust_pizza: super::Pizza) {
            // `{}` placeholder formats the value using Display trait.
            // `cust_pizza.topping` accesses the topping field using dot notation.
            println!("Serving pizza with {} topping.", cust_pizza.topping);
        }
    }
}

// `pub fn` defines a public function at the crate root level.
// This function is the public API for ordering food.
// Can be called from other modules or external crates.
pub fn order_food() {
    // `crate::` is an absolute path starting from the crate root.
    // `crate` keyword refers to the root of the current crate (binary or library).
    // Full path: crate → restaurant → pizza_order → help_customer → take_order
    // NOTE: This assumes the module is inside a `restaurant` module at crate root.
    crate::restaurant::pizza_order::help_customer::take_order();
}

// =============================================================================
// KEY CONCEPTS EXPLAINED:
// =============================================================================
//
// 1. MODULES (`mod`):
//    - Organize code into namespaces
//    - Control visibility and encapsulation
//    - Can be nested (modules within modules)
//    - Default: everything is private
//
// 2. VISIBILITY (`pub`):
//    - `pub` makes items visible outside their module
//    - Without `pub`, items are private to their module
//    - Privacy rules:
//      * Struct: `pub struct` makes type visible, fields still private
//      * Fields: Need `pub` on each field to make them accessible
//      * Functions: `pub fn` makes function callable from outside
//
// 3. MODULE PATHS:
//    - `::` separates path components (like `/` in file paths)
//    - `crate::` starts from crate root (absolute path)
//    - `super::` refers to parent module (relative path)
//    - `self::` refers to current module (usually implicit)
//
// 4. PATH EXAMPLES:
//    - Absolute: `crate::pizza_order::Pizza`
//    - Relative: `super::Pizza` (from help_customer to pizza_order)
//    - Nested: `pizza_order::help_customer::take_order`
//
// 5. ASSOCIATED FUNCTIONS VS METHODS:
//    - Associated function: No `self` parameter
//      Example: `Pizza::lunch(...)` - called on type
//    - Method: Has `self`, `&self`, or `&mut self`
//      Example: `pizza.eat()` - called on instance
//
// 6. OWNERSHIP IN PARAMETERS:
//    - `serve_customer(cust_pizza: Pizza)` takes ownership
//    - After call, cust_pizza can't be used by caller
//    - Alternative: `&Pizza` borrows immutably
//    - Alternative: `&mut Pizza` borrows mutably
//
// 7. STRING TYPES:
//    - `String`: Owned, heap-allocated, growable
//    - `&str`: Borrowed string slice, immutable view
//    - `String::from("text")` converts &str to String
//    - `.to_string()` is another way to convert
//
// 8. MODULE STRUCTURE ISSUE:
//    The code has a path mismatch:
//    - order_food() calls: crate::restaurant::pizza_order::...
//    - But pizza_order is defined at crate root, not in restaurant
//    
//    To fix, either:
//    A) Wrap in restaurant module:
//       mod restaurant {
//           pub mod pizza_order { ... }
//       }
//    
//    B) Change the path in order_food():
//       crate::pizza_order::help_customer::take_order();
//
// 9. PRIVACY BOUNDARIES:
//    - seat_at_table(): Private, only help_customer can call
//    - serve_customer(): Private, only help_customer can call
//    - take_order(): Public, can be called from anywhere
//    - Pizza struct: Public, but if fields weren't pub, only lunch() could set them
//
// 10. TYPICAL MODULE PATTERNS:
//     - File-based: Each module in separate file (mod.rs or module_name.rs)
//     - Inline: Modules defined in same file (as shown here)
//     - Nested: Submodules for organizing related functionality
//     - Re-exports: Use `pub use` to flatten deep hierarchies
//
// =============================================================================

// CORRECTED VERSION (fixing the path issue):
/*
mod restaurant {
    pub mod pizza_order {
        pub struct Pizza {
            pub dough: String,
            pub cheese: String,
            pub topping: String,
        }
        
        impl Pizza {
            pub fn lunch(topping: String) -> Pizza {
                Pizza {
                    dough: String::from("regular crust"),
                    cheese: String::from("mozzarella"),
                    topping,
                }
            }
        }

        pub mod help_customer {
            fn seat_at_table() {
                println!("Customer seated at table.");
            }

            pub fn take_order() {
                seat_at_table();
                let customer_order = super::Pizza::lunch(String::from("veggies"));
                serve_customer(customer_order);
            }
            
            fn serve_customer(cust_pizza: super::Pizza) {
                println!("Serving pizza with {} topping.", cust_pizza.topping);
            }
        }
    }
}

pub fn order_food() {
    // Now this path is correct
    crate::restaurant::pizza_order::help_customer::take_order();
}

fn main() {
    order_food();
}
*/