extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate orange_db_binary_serde_traits;

use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Ident, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics, Index};


#[proc_macro_derive(BinarySer)]
pub fn binary_serialize(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);
    
    let ser_expr = binary_serialize_struct_fields(&parsed_input.data);

    let name = &parsed_input.ident;
    
    let expanded = quote!(
        impl BinarySer for #name {
            fn ser(&self) -> Vec<u8> {
                #ser_expr
            }
        }
    );
    proc_macro::TokenStream::from(expanded)
}

fn binary_serialize_struct_fields(data: &Data) -> proc_macro2::TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let recurse = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        quote_spanned! {f.span() =>
                            res.append(&mut self.#name.to_be_bytes().to_vec());
                        }
                    });
                    quote! {
                        let mut res = Vec::new(); #(#recurse;)* res
                    }
                }
                Fields::Unnamed(ref fields) => {
                    let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                        let index = Index::from(i);
                        quote_spanned! {f.span() =>
                            res.append(self.#index.to_be_bytes().to_vec());
                        }
                    });
                    quote! {
                        let mut res = Vec::new(); #(#recurse;)* res
                    }
                }
                Fields::Unit => {
                    quote!(0)
                }
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}


#[proc_macro_derive(BinaryDeser)]
pub fn binary_deserialize(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);
    
    let name = &parsed_input.ident;
    let deser_expr = binary_deserialize_struct(&parsed_input.data, &name);

    
    let expanded = quote!(
        impl BinaryDeser for #name {
            fn deser(bytes: &Vec<u8>) -> #name {
                #deser_expr
            }
        }
    );
    proc_macro::TokenStream::from(expanded)
}


fn binary_deserialize_struct(data: &Data, s_name: &Ident) -> proc_macro2::TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let recurse = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        let ty = &f.ty;
                        quote_spanned! {f.span() =>
                            #name: {const ty_len: usize = std::mem::size_of::<#ty>(); let mut tmp = [0u8; ty_len]; tmp.copy_from_slice(&bytes[index..index+ty_len]); index += ty_len; #ty::from_be_bytes(tmp)}
                        }
                    });
                    quote! {
                        let mut index = 0;
                        #s_name{#(#recurse,)*}
                    }
                }
                Fields::Unnamed(ref fields) => {
                    let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                        let index = Index::from(i);
                        let ty = &f.ty;
                        quote_spanned! {f.span() =>
                            {const ty_len: usize = std::mem::size_of::<#ty>(); let mut tmp = [0u8; ty_len]; tmp.copy_from_slice(&bytes[index..index+ty_len]); index += ty_len; #ty::from_be_bytes(tmp)}
                        }
                    });
                    quote! {
                        let mut index = 0;
                        #s_name(#(#recurse,)*)
                    }
                }
                Fields::Unit => {
                    quote!(0)
                }
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}