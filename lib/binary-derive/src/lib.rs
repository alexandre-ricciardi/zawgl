extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Ident, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics, Index};


#[proc_macro_derive(Binarize)]
pub fn binary_serialize(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);
    
    let name = &parsed_input.ident;
    let ser_expr = binary_serialize_struct_fields(&parsed_input.data, &name);
    let deser_expr = binary_deserialize_struct(&parsed_input.data, &name);
    
    let expanded = quote!(
        impl #name {
            pub const SIZE: usize = std::mem::size_of::<#name>();
            pub fn to_bytes(&self) -> [u8; std::mem::size_of::<#name>()] {
                #ser_expr
            }
            pub fn from_bytes(bytes: [u8; std::mem::size_of::<#name>()]) -> #name {
                #deser_expr
            }
        }
    );
    proc_macro::TokenStream::from(expanded)
}

fn binary_serialize_struct_fields(data: &Data, s_name: &Ident) -> proc_macro2::TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let recurse = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        let ty = &f.ty;
                        quote_spanned! {f.span() =>
                            bytes[index..index+std::mem::size_of::<#ty>()].copy_from_slice(&self.#name.to_be_bytes());index += std::mem::size_of::<#ty>();
                        }
                    });
                    quote! {
                        let mut index = 0;
                        let mut bytes = [0u8; Self::SIZE];
                        #(#recurse)* bytes
                    }
                },
                Fields::Unnamed(ref fields) => {
                    let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                        let index = Index::from(i);
                        let ty = &f.ty;
                        quote_spanned! {f.span() =>
                            bytes[index..index + std::mem::size_of::<#ty>()].copy_from_slice(&self.#index.to_be_bytes());
                            index += std::mem::size_of::<#ty>();
                        }
                    });
                    quote! {
                        let mut index = 0;
                        let mut bytes = [0u8; std::mem::size_of::<#s_name>()];
                        #(#recurse)* bytes
                    }
                },
                _ => unimplemented!()
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
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
                            #name: {const TY_LEN: usize = std::mem::size_of::<#ty>(); let mut tmp = [0u8; TY_LEN]; tmp.copy_from_slice(&bytes[index..index+TY_LEN]); index += TY_LEN; #ty::from_be_bytes(tmp)}
                        }
                    });
                    quote! {
                        let mut index = 0;
                        #s_name{#(#recurse,)*}
                    }
                },
                Fields::Unnamed(ref fields) => {
                    let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                        let index = Index::from(i);
                        let ty = &f.ty;
                        quote_spanned! {f.span() =>
                            {const TY_LEN: usize = std::mem::size_of::<#ty>(); let mut tmp = [0u8; TY_LEN]; tmp.copy_from_slice(&bytes[index..index+TY_LEN]); index += TY_LEN; #ty::from_be_bytes(tmp)}
                        }
                    });
                    quote! {
                        let mut index = 0;
                        #s_name(#(#recurse,)*)
                    }
                },
                _ => unimplemented!()
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}