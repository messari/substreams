use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{DataEnum, Fields};

use crate::modified_rand_gen::parser::{attrs_to_customizes, has_customize, Customize};
use crate::modified_rand_gen::gen::{
    generated_values_for_named_fields, generated_values_for_unnamed_fields, TraitMethods,
};

pub fn generate(name: &Ident, trait_methods: &mut TraitMethods, de: DataEnum) -> TokenStream {
    let variants = de
        .variants
        .into_iter()
        // Filter out variants annotated with SkipVariant
        .filter(|v| {
            let customizes = attrs_to_customizes(&v.attrs);

            !has_customize(&customizes, Customize::Skip)
        })
        .collect::<Vec<_>>();
    let variants_len = variants.len();
    let range: Vec<_> = (0..variants_len).collect();

    let ts = variants
        .into_iter()
        .map(|v| {
            let fields = v.fields;
            let ident = v.ident;
            let prefix = quote! {
                #name::#ident
            };

            if fields.is_empty() {
                prefix
            } else {
                match fields {
                    Fields::Named(n) => {
                        let ts = generated_values_for_named_fields(name, n, trait_methods);

                        quote! {
                            #prefix { #(#ts),* }
                        }
                    }
                    Fields::Unnamed(u) => {
                        let ts = generated_values_for_unnamed_fields(name, u, trait_methods);

                        quote! {
                            #prefix (#(#ts),* )
                        }
                    }
                    Fields::Unit => panic!(),
                }
            }
        })
        .collect::<Vec<_>>();
    quote! {
        let random_val = rng.gen_range(0..#variants_len);

        match random_val {
            #(#range => #ts,)*
            _ => panic!()
        }
    }
}
