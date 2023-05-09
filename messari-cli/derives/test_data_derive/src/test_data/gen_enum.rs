use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{DataEnum, Fields};

pub fn generate(name: &Ident, de: DataEnum, starting_tag: u8) -> TokenStream {

    quote! {

    }
}
