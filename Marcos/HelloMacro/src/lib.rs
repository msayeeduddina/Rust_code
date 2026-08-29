/// # HelloMacros Trait
///
/// A **custom derive trait** — any struct that `#[derive(HelloMacros)]` will
/// automatically get a `hello_marco()` method generated at compile time via
/// the proc-macro in `hello_macro_derive`.
///
/// ## Key concept
/// This is the *trait definition* crate. It only declares the interface.
/// The actual code generation lives in the separate `hello_macro_derive` crate
/// (Rust requires proc-macro crates to be their own crate).
///
/// ## Default impl
/// The default body here is a fallback — the derive macro **overrides** it
/// with a type-aware version that prints the struct name.
pub trait HelloMacros {
    fn hello_marco() {
        println!("Hello World this is Pancake");
    }
}
