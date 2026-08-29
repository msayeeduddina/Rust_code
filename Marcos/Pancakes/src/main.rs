// `hello_macro_derive` — brings the `#[derive(HelloMacros)]` attribute into scope
// `hello_macro`         — brings the `HelloMacros` trait into scope (needed to call its methods)
// Both imports are required; the derive crate alone doesn't expose the trait.
use hello_macro::HelloMacros;
use hello_macro_derive::HelloMacros;

/// At compile time, the proc-macro generates:
/// ```
/// impl HelloMacros for Pancakes {
///     fn hello_marco() { println!("Hello Marco from Pancakes!"); }
/// }
/// ```
#[derive(HelloMacros)]
struct Pancakes;

// Same code generation happens for every struct that derives HelloMacros —
// each gets its own impl with its own type name baked in via stringify!
#[derive(HelloMacros)]
struct Pancakes2;

#[derive(HelloMacros)]
enum Pancakes3 {}

fn main() {
    Pancakes::hello_marco(); // → "Hello Marco from Pancakes!"
    Pancakes2::hello_marco(); // → "Hello Marco from Pancakes2!"
    Pancakes3::hello_marco(); // → "Hello Marco from Pancakes3!"
}
