mod test_data;

use syn::DeriveInput;

#[proc_macro_derive(TestData, attributes(proto_type, starting_tag))]
pub fn test_data(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();

    let transform = test_data::test_data(input);

    transform.into()
}
