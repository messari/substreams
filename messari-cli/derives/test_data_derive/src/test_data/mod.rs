mod gen;
mod gen_enum;
mod gen_struct;
mod proto_alternative_type;

use proc_macro2::TokenStream;
use syn::DeriveInput;

pub(crate) fn test_data(input: DeriveInput) -> TokenStream {
    gen::transform(input)
}