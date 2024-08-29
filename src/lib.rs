#![crate_name = "small_read_only"]
//! ## Example
//! 
//! ```rust
//! use small_read_only::ReadOnly;
//! 
//! #[derive(ReadOnly)]
//! pub struct A<'a> {
//!     b: usize,
//!     c: String,
//!     d: &'a str,
//! }
//! 
//! impl<'a> A<'a> {
//!     pub fn new(b: usize, c: String, d: &'a str) -> Self {
//!         Self {
//!             b, c, d
//!         }
//!     }
//! }
//! 
//! let a = A::new(1, "c".to_string(), "d");
//! 
//! assert_eq!(a.b(), &1);
//! assert_eq!(a.c(), "c");
//! assert_eq!(a.d(), &"d");
//! ```
//! # NoRead attribute
//! ```compile_fail
//! #[derive(ReadOnly)]
//! pub struct B {
//!     #[NoRead]
//!     b: usize,
//! }
//! 
//! impl B {
//!     pub fn new(b: usize) -> Self {
//!         Self {
//!             b
//!         }
//!     }
//! }
//! 
//! let b = B::new(1);
//! b.b();
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

/// Implement the getters for all fields of a struct
#[proc_macro_derive(ReadOnly, attributes(NoRead))]
pub fn derive_read_only(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let generics = input.generics;

    let expanded = match input.data {
        Data::Struct(data_struct) => {
            match data_struct.fields {
                Fields::Named(fields) => {
                    let fields = fields.named.into_iter()
                        .filter(|field| {
                            let write = &field.attrs.iter().any(|attr| {
                                attr.path().is_ident("NoRead")
                            });

                            !write
                        }).collect::<Vec<_>>();  
                    
                    let functions = fields.into_iter().map(|field| {
                        let field_name = field.ident.unwrap();
                        let field_type = field.ty;

                        quote! {
                            impl #generics #name #generics {
                                /// Derived from ReadOnly
                                pub fn #field_name(&self) -> &#field_type {
                                    &self.#field_name
                                }
                            }
                        }
                    }).collect::<Vec<_>>();

                    quote! {
                        #(#functions)*
                    }
                },
                _ =>  panic!("ReadOnly not implemented for unit structs")
            }
        },
        _ => panic!("ReadOnly only implemented for structs/tuples")
    };

    expanded.into()
}

