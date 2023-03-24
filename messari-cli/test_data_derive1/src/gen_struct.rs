use crate::gen::{
    generated_values_for_named_fields, generated_values_for_unnamed_fields, TraitMethods,
};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{DataStruct, Fields};

pub fn generate(name: &Ident, trait_methods: &mut TraitMethods, ds: DataStruct) -> TokenStream {
    match ds.fields {
        Fields::Named(n) => {
            let ts = generated_values_for_named_fields(name, n, trait_methods);

            quote! {
                #name { #(#ts),* }
            }
        }
        Fields::Unnamed(u) => {
            let ts = generated_values_for_unnamed_fields(name, u, trait_methods);

            quote! {
                #name (#(#ts),* )
            }
        }
        Fields::Unit => quote! {
            #name
        },
    }
}
