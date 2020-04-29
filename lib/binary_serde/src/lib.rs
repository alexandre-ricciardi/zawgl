extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;


use proc_macro::TokenStream;
use syn::DeriveInput;
use syn::parse_macro_input;


#[proc_macro_derive(BinarySerialize)]
pub fn binary_serialize(input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(input as DeriveInput);

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        impl BinarySerialize for #name {
            fn to_binary(bytes: [u8]) {
                println!("Hello, World! My name is {}", stringify!(#name));
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
