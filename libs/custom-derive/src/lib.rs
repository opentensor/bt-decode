use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse::Nothing, parse2, parse_quote, Error, ItemImpl, Result};

/// Automatically adds `py_decode` and `py_decode_vec` methods to a struct's inherent impl block,
/// making them available as Python methods via `pyo3`.
///
/// ```ignore
/// use your_crate::pydecode;
///
/// #[pydecode]
/// #[pymethods]
/// impl MyStruct {
///     // Additional methods can be added here
/// }
/// ```
#[proc_macro_attribute]
pub fn pydecode(attr: TokenStream, tokens: TokenStream) -> TokenStream {
    match pydecode_impl(attr.into(), tokens.into()) {
        Ok(item_impl) => item_impl.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn pydecode_impl(attr: TokenStream2, tokens: TokenStream2) -> Result<TokenStream2> {
    parse2::<Nothing>(attr)?;
    let mut item_impl = parse2::<ItemImpl>(tokens)?;

    // Ensure the impl block is not for a trait
    if item_impl.trait_.is_some() {
        return Err(Error::new_spanned(
            &item_impl,
            "The pydecode macro should only be applied to inherent impl blocks.",
        ));
    }

    // Extract the struct name from the impl block's self type
    let struct_name = if let syn::Type::Path(ref path) = *item_impl.self_ty {
        &path.path.segments.last().unwrap().ident
    } else {
        return Err(Error::new_spanned(
            item_impl.self_ty,
            "Expected a struct name in the impl block.",
        ));
    };

    let struct_name_str = struct_name.to_string();

    // Add the py_decode method
    item_impl.items.push(parse_quote! {
        #[pyo3(name = "decode")]
        #[staticmethod]
        fn py_decode(encoded: &[u8]) -> Self {
            #struct_name::decode(&mut &encoded[..])
                .expect(&format!("Failed to decode {}", #struct_name_str))
        }
    });

    // Add the py_decode_vec method
    item_impl.items.push(parse_quote! {
        #[pyo3(name = "decode_vec")]
        #[staticmethod]
        fn py_decode_vec(encoded: &[u8]) -> Vec<Self> {
            Vec::<#struct_name>::decode(&mut &encoded[..])
                .expect(&format!("Failed to decode Vec<{}>", #struct_name_str))
        }
    });

    // Add the py_decode_option method
    item_impl.items.push(parse_quote! {
        #[pyo3(name = "decode_option")]
        #[staticmethod]
        fn py_decode_option(encoded: &[u8]) -> Option<Self> {
            Option::<#struct_name>::decode(&mut &encoded[..])
                .expect(&format!("Failed to decode Option<{}>", #struct_name_str))
        }
    });

    Ok(quote!(#item_impl))
}

// Inline tests
#[test]
fn test_pydecode_macro() {
    let input = quote! {
        #[pymethods]
        impl MyStruct {
            // Other methods
        }
    };

    let expected = quote! {
        #[pymethods]
        impl MyStruct {
            // Other methods

            #[pyo3(name = "decode")]
            #[staticmethod]
            fn py_decode(encoded: &[u8]) -> Self {
                let decoded = MyStruct::decode(&mut &encoded[..])
                    .expect("Failed to decode");
                decoded
            }

            #[pyo3(name = "decode_vec")]
            #[staticmethod]
            fn py_decode_vec(encoded: &[u8]) -> Vec<Self> {
                Vec::<MyStruct>::decode(&mut &encoded[..])
                    .expect(&format!("Failed to decode Vec<{}>", "MyStruct"))
            }
        }
    };

    let output = pydecode_impl(TokenStream2::new(), input).unwrap();

    assert_eq!(output.to_string(), expected.to_string());
}

#[test]
fn test_pydecode_with_trait_impl_error() {
    let input = quote! {
        impl MyTrait for MyStruct {
            // Methods
        }
    };

    let result = pydecode_impl(TokenStream2::new(), input);

    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(
            err.to_string(),
            "The pydecode macro should only be applied to inherent impl blocks."
        );
    }
}
