use crate::parser::Customize::{AlwaysNone, AlwaysSome, Custom, Empty, Panic, Skip};
use proc_macro2::TokenStream;
#[allow(unused_imports)]
use std::str::FromStr;
use syn::parse::{Parse, ParseBuffer};
use syn::Attribute;

#[allow(clippy::ptr_arg)]
pub(crate) fn attrs_to_customizes(attrs: &Vec<Attribute>) -> Vec<Customize> {
    attrs
        .clone()
        .into_iter()
        .filter_map(|a| {
            if !proc_macro2_helper::attribute_contains(&a, "rand_derive") {
                return None;
            }

            let clone: proc_macro::TokenStream = a.tokens.into();

            Some(syn::parse2::<Customize>(clone.into()).unwrap())
        })
        .collect()
}

struct Fixed {
    ident: String,
    stream: Option<String>,
}

impl Parse for Fixed {
    fn parse(input: &ParseBuffer) -> syn::Result<Self> {
        let ident: proc_macro2::Ident = syn::parse::Parse::parse(input)?;
        let ident = ident.to_string();

        if ident.as_str() != "fixed" {
            return Ok(Fixed {
                ident,
                stream: None,
            });
        }

        // This is the equal sign
        let _: proc_macro2::Punct = syn::parse::Parse::parse(input)?;

        // This is the actual value
        let lit: proc_macro2::Literal = syn::parse::Parse::parse(input)?;

        Ok(Fixed {
            ident,
            // There are still leading and trailing quotes, this needs to be removed
            stream: Some(lit.to_string().replace('\"', "")),
        })
    }
}

#[derive(PartialEq, Clone, Debug)]
pub(crate) enum Customize {
    Skip,
    AlwaysSome,
    AlwaysNone,
    Custom,
    Panic,
    Default,
    Empty,
    Fixed(String),
}

impl Parse for Customize {
    fn parse(input: &ParseBuffer) -> syn::Result<Self> {
        let group = proc_macro2::Group::parse(input).unwrap();
        let fixed = syn::parse2::<Fixed>(group.stream())?;

        Ok(match fixed.stream {
            None => match fixed.ident.as_str() {
                "skip" => Skip,
                "some" => AlwaysSome,
                "none" => AlwaysNone,
                "custom" => Custom,
                "panic" => Panic,
                "default" => Customize::Default,
                "empty" => Empty,
                "fixed" => unreachable!(),
                _ => panic!("Unknown customization: {}", fixed.ident),
            },
            Some(stream) => Customize::Fixed(stream),
        })
    }
}

pub(crate) fn has_customize(customizes: &[Customize], customize: Customize) -> bool {
    customizes.iter().any(|c| c == &customize)
}

pub(crate) fn fixed_value(customizes: &[Customize]) -> Option<TokenStream> {
    customizes.iter().find_map(|c| {
        if let Customize::Fixed(fixed) = c {
            Some(TokenStream::from_str(fixed).unwrap())
        } else {
            None
        }
    })
}
