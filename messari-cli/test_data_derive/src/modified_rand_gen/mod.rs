mod gen;
mod gen_enum;
mod gen_struct;
mod parser;

use proc_macro2::TokenStream;
use syn::DeriveInput;

pub(crate) fn rand_gen(input: DeriveInput) -> TokenStream {
    gen::transform(input)
}
