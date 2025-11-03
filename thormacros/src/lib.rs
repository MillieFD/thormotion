/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::spanned::Spanned;
use syn::{Attribute, FnArg, ItemFn, Pat, PatIdent, parse_macro_input};

/// Attribute macro to convert a single proto-function definition into:
///
/// 1. A user-friendly synchronous function
/// 2. An async function for advanced users
///
/// Synchroneity is achieved by wrapping the core async function inside `smol::block_on`.
///
/// ### Example
///
/// ```rust
/// #[thormacros::sync]
/// #[doc = include_str!("../documentation/move_absolute.md")]
/// pub async fn move_absolute_async(&self, position: f64) {
///     functions::move_absolute(self, 1, position).await;
/// }
/// ```
///
/// The proto-function definition above expands to `move_absolute_async` and `move_absolute`.
#[proc_macro_attribute]
pub fn sync(_attr: TokenStream, ts: TokenStream) -> TokenStream {
    // 1. Parse the annotated item into a typed AST node (`ItemFn`).
    let item = parse_macro_input!(ts as ItemFn);

    // 2. Ensure the annotated item is an async function.
    if item.sig.asyncness.is_none() {
        return syn::Error::new(item.sig.span(), "#[sync] requires an async fn")
            .to_compile_error()
            .into();
    }

    // 3. Extract components from the parsed input

    // 3a. Function visibility e.g. pub(crate)
    let vis = &item.vis;

    // 3b. Function attributes e.g. #[doc = "foo"]
    let attrs: Vec<Attribute> = item.attrs.into_iter().filter(not_sync_attr).collect();

    // 3c. Function name e.g. foo_async
    let async_name = &item.sig.ident;

    // 3d. Function input(s) name(s) and type(s) e.g. a: f64, b: bool
    let inputs = &item.sig.inputs;

    // 3e. Function output type e.g. i32
    let output = &item.sig.output;

    // 3f. Generics and lifetimes (if any)
    let generics = &item.sig.generics;

    // 3g. Where clause (if any)
    let where_clause = &item.sig.generics.where_clause;

    // 3h. Function body i.e. internal logic
    let body = &item.block;

    // 4. Ensure the function name ends with `_async`.
    let sync_name = {
        let tmp = async_name.to_string();
        match tmp.strip_suffix("_async") {
            None => {
                return syn::Error::new(
                    item.sig.ident.span(),
                    "#[sync] function name must include the `_async` suffix.\nDefine `async fn \
                     foo_async(...)` (with suffix); the #[sync] macro will generate a synchronous \
                     `fn foo(...)` wrapper.",
                )
                .to_compile_error()
                .into();
            }
            Some(s) => syn::Ident::new(s, item.sig.ident.span()),
        }
    };

    // 5. Extract input names (ignore types) for forwarding through the generated wrapper function
    let mut receiver = false;
    let mut args = Vec::new();

    for input in inputs.iter() {
        match input {
            FnArg::Receiver(_) => {
                // Function input is &self or &mut self. Exclude from args list.
                receiver = true;
            }
            FnArg::Typed(t) => {
                // Function input is typed (e.g. x: T). Check the pattern is valid.
                if let Pat::Ident(PatIdent { ident, .. }) = &*t.pat {
                    // Add input identifier (name) to args list
                    args.push(ident);
                } else {
                    // Raise compiler error
                    return syn::Error::new(
                        t.span(),
                        "#[sync] currently only supports simple identifier arguments like `x: T`.",
                    )
                    .to_compile_error()
                    .into();
                }
            }
        }
    }

    // 6. Build the call target for the sync wrapper
    let caller = match receiver {
        true => quote! { self. }, // Instance functions
        false => quote! {},       // Static functions
    };
    // WARN: No support for Self:: functions yet

    // 7. Generate the async underlying function as a token stream
    let async_fn = quote! {
        #(#attrs)*
        #vis async fn #async_name #generics (#inputs) #output #where_clause {
            #body
        }
    };

    // 8. Generate the sync wrapper function as a token stream
    let sync_fn = quote! {
        #(#attrs)*
        #vis fn #sync_name #generics (#inputs) #output #where_clause {
            // The sync function runs the async function to completion on the current thread.
            ::smol::block_on(async { #caller #async_name ( #(#args),* ).await })
        }
    };

    // 9. Return both functions to the compiler as the expansion result.
    TokenStream::from(quote! {
        #async_fn
        #sync_fn
    })
}

/// Returns `True` if the attribute is `#[sync]`.
///
/// It is essential to filter out our `#[sync]` attribute to prevent recursively duplicating the
/// macro onto generated functions.
///
/// All other attributes (e.g. `docs` or `cfg` flags) are duplicated onto the generated function.
fn not_sync_attr(attr: &Attribute) -> bool {
    // Ignores optional path prefixes e.g. `thormacros::sync`
    !attr.path().to_token_stream().to_string().ends_with("sync")
}
