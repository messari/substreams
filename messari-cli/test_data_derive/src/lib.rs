mod modified_rand_gen;
mod test_data;

use syn::DeriveInput;

use crate::modified_rand_gen::rand_gen;

#[proc_macro_derive(TestData, attributes(proto_type, starting_tag))]
pub fn test_data(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();

    let rand_gen_derive = rand_gen(input.clone());

    let transform = test_data::test_data(input);

    // transform.into()
    rand_gen_derive.into()
}
