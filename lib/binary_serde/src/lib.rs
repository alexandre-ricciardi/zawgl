extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate orange_db_binary_serde_traits;


use proc_macro::TokenStream;
use syn::DeriveInput;
use syn::parse_macro_input;


#[proc_macro_derive(BinarySer)]
pub fn binary_serialize(input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);
    let name = &parsed_input.ident;
    let expanded = quote!(
        impl BinarySer for #name {
            fn ser(&self) -> Vec<u8> {
                Vec::new()
            }
        }
    );
    TokenStream::from(expanded)
}
