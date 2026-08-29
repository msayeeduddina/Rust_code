use proc_macro::TokenStream;
use quote::quote;
use syn;

/// # `#[derive(HelloMacros)]` — Procedural Macro Entry Point
///
/// ## How proc-macro derives work (revision notes)
/// 1. Rust compiler hands us the **token stream** of the annotated item.
/// 2. `syn::parse` turns raw tokens → a typed AST (`DeriveInput`).
/// 3. `quote!` turns Rust code written as a template → new tokens.
/// 4. We return those tokens; the compiler splices them into the crate.
///
/// ## Crate split rule
/// Proc-macro crates MUST be their own crate with `proc-macro = true` in
/// Cargo.toml. They cannot export normal items alongside proc-macros.
#[proc_macro_derive(HelloMacros)]
pub fn hello_macro_derive(item: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree (DeriveInput = struct/enum/union)
    let ast = syn::parse(item).unwrap();
    impl_hello_macro(&ast)
}

/// Generates the `impl HelloMacros for <Name>` block.
///
/// `ast.ident` — the identifier (name) of the annotated struct/enum.
/// `stringify!(#name)` — turns the ident into a `&str` *at runtime*.
/// `quote!` — quasi-quoting macro; `#name` is interpolation (like format!).
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident; // e.g. `Pancakes`
    let r#gen = quote! {
        // This block is injected into the caller's crate at compile time
        impl HelloMacros for #name {
            fn hello_marco() {
                println!("Hello Marco from {}!", stringify!(#name));
            }
        }
    };
    r#gen.into() // Convert quote's TokenStream2 → proc_macro::TokenStream
}
