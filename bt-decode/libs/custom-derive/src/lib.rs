extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, token::Token, ImplItem};

#[proc_macro_attribute]
pub fn pydecode(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut ast = parse_macro_input!(item as syn::ItemImpl);
    let struct_name = &ast.trait_.as_ref().unwrap().1;

    ast.items.push(syn::parse::<ImplItem>(
        quote!(
            #[pyo3(name = "decode")]
            #[staticmethod]
            fn py_decode(encoded: &[u8]) -> Self {
                let decoded = #struct_name::decode(&mut &encoded[..])
                    .expect("Failed to decode");
                decoded
            } ).into()
    ).unwrap());

    ast.to_token_stream().into()
}