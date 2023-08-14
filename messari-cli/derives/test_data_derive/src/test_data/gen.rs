use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Attribute, Data, DeriveInput, LitInt};
use syn::parse::Parse;
use syn::parse::ParseBuffer;

use crate::test_data::{gen_enum, gen_struct};

pub(crate) fn transform(input: DeriveInput) -> TokenStream {
    let name = &input.ident;
    let starting_tag = get_starting_tag(&input.attrs);

    match input.data {
        Data::Struct(ds) => gen_struct::generate(name, ds, starting_tag),
        Data::Enum(de) => gen_enum::generate(name, de, starting_tag),
        Data::Union(_) => panic!("Unions are currently not supported"),
    }
}

fn get_starting_tag(attributes: &Vec<Attribute>) -> u8 {
    let mut starting_tags = attributes.iter().filter_map(|attribute| {
        if !proc_macro2_helper::attribute_contains(attribute, "starting_tag") {
            return None;
        }

        let starting_tag = syn::parse2::<StartingTag>(attribute.to_token_stream()).expect("treat_as_type value given is incorrect!");

        Some(starting_tag.0)
    }).collect::<Vec<_>>();

    match starting_tags.len() {
        0 => 1,
        1 => starting_tags.pop().unwrap(),
        _ => panic!("More than one starting tag attribute given! Only specify either 0 or 1 starting tag attributes!")
    }
}

struct StartingTag(u8);

impl Parse for StartingTag {
    fn parse(input: &ParseBuffer) -> syn::Result<Self> {
        let group = proc_macro2::Group::parse(input).unwrap();
        let tag_literal = syn::parse2::<LitInt>(group.stream()).expect("Unable to parse tag literal!");
        let tag_number = tag_literal.base10_parse::<u8>().expect(&format!("Tag value given is not u8 compatible!: {}", tag_literal));

        Ok(StartingTag(tag_number))
    }
}