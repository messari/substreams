mod gen;
mod gen_enum;
mod gen_struct;
mod parser;
mod proto_alternative_type;

use proc_macro2::TokenStream;
use syn::DeriveInput;

pub(crate) fn test_data(input: DeriveInput) -> TokenStream {
    // let data = input.data;
    //
    // let data_struct = if let Data::Struct(data_struct) = data {
    //     data_struct
    // } else {
    //     unreachable!()
    // };
    //
    // panic!("Name: {}\n\n{:?}\n\n", name, data_struct.fields.iter().next().unwrap());

    gen::transform(input)
}