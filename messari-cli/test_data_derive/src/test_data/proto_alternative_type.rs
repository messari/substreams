use syn::Token;
use proc_macro2::{Ident, TokenStream};
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::punctuated::Punctuated;
use crate::test_data::gen_struct::{FieldAssociation, ParquetType};

pub(crate) fn parse_proto_alternate_type(field: &syn::Field) -> Option<ProtoAlternativeType> {
    let mut proto_alternative_types = field.attrs.iter().filter_map(|attribute| {
        if !proc_macro2_helper::attribute_contains(&attribute, "proto_type") {
            return None;
        }

        Some(syn::parse2::<ProtoAlternativeType>(attribute.tokens.clone()).expect("treat_as_type value given is incorrect!"))
    }).collect::<Vec<_>>();

    match proto_alternative_types.len() {
        0 => None,
        1 => Some(proto_alternative_types.pop().unwrap()),
        _ => panic!("More than one alternative type specified!: {:?}. Please specify either 0 or one types!", proto_alternative_types),
    }
}

#[derive(PartialEq, Clone, Debug)]
pub(crate) enum ProtoAlternativeType {
    Fixed64,
    Fixed32,
    Sfixed32,
    Sfixed64,
    Sint32,
    Sint64,
    Enum,
    Oneof(Vec<Ident>)
}

impl ProtoAlternativeType {
    pub(crate) fn is_oneof_type(&self) -> bool {
        if let ProtoAlternativeType::Oneof(_) = self {
            true
        } else {
            false
        }
    }

    pub(crate) fn get_num_tags(&self) -> u8 {
        if let ProtoAlternativeType::Oneof(field_idents) = self {
            field_idents.len() as u8
        } else {
            1
        }
    }

    pub(crate) fn get_parquet_type(&self, inner_type: &str) -> ParquetType {
        match self {
            ProtoAlternativeType::Fixed64 => ParquetType::ULong,
            ProtoAlternativeType::Fixed32 => ParquetType::UInt,
            ProtoAlternativeType::Sfixed32 => ParquetType::Int,
            ProtoAlternativeType::Sfixed64 => ParquetType::Long,
            ProtoAlternativeType::Sint32 => ParquetType::Int,
            ProtoAlternativeType::Sint64 => ParquetType::Long,
            ProtoAlternativeType::Enum => ParquetType::Enum(inner_type.to_string()),
            _ => unreachable!()
        }
    }

    pub(crate) fn get_field_assocation(&self, field_name: String) -> Option<FieldAssociation> {
        if let ProtoAlternativeType::Oneof(field_idents) = self {
            Some(FieldAssociation::from_oneof_field_and_type_idents(field_name, field_idents))
        } else {
            None
        }
    }

    pub(crate) fn get_proto_type(&self, inner_type: &str) -> String {
        match self {
            ProtoAlternativeType::Fixed64 => "fixed64".to_string(),
            ProtoAlternativeType::Fixed32 => "fixed32".to_string(),
            ProtoAlternativeType::Sfixed32 => "sfixed32".to_string(),
            ProtoAlternativeType::Sfixed64 => "sfixed64".to_string(),
            ProtoAlternativeType::Sint32 => "sint32".to_string(),
            ProtoAlternativeType::Sint64 => "sint64".to_string(),
            ProtoAlternativeType::Enum => format!("enumeration=\"{}\"", inner_type),
            ProtoAlternativeType::Oneof => format!("oneof=\"{}\"", inner_type),
        }
    }
}

impl Parse for ProtoAlternativeType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let group = proc_macro2::Group::parse(input).unwrap();
        Ok(syn::parse2::<ProtoTypeInfo>(group.stream()).unwrap().0)
    }
}

struct ProtoTypeInfo(ProtoAlternativeType);

impl Parse for ProtoTypeInfo {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        let ident_string = ident.to_string();
        if ident_string=="oneof" {
            let group = proc_macro2::Group::parse(input).unwrap();
            let field_idents = match syn::parse2::<OneofFieldIdents>(group.stream()) {
                Ok(oneofFieldIdents) => oneofFieldIdents.0,
                Err(_) => panic!("TODO@!!123")
            };

            if field_idents.len() == 0 {
                panic!("No fields were specified for oneof type! Please specify at least 2 field names for a oneof type.");
            }

            if field_idents.len() == 1 {
                panic!("Only 1 field was specified for oneof type! Please specify at least 2 field names for a oneof type. Field name specified: {}", field_idents.first().unwrap().to_string());
            }

            Ok(ProtoTypeInfo(ProtoAlternativeType::Oneof(field_idents)))
        } else {
            Ok(match ident_string.as_str() {
                "fixed32" => ProtoTypeInfo(ProtoAlternativeType::Fixed32),
                "fixed64" => ProtoTypeInfo(ProtoAlternativeType::Fixed64),
                "sfixed32" => ProtoTypeInfo(ProtoAlternativeType::Sfixed32),
                "sfixed64" => ProtoTypeInfo(ProtoAlternativeType::Sfixed64),
                "sint32" => ProtoTypeInfo(ProtoAlternativeType::Sint32),
                "sint64" => ProtoTypeInfo(ProtoAlternativeType::Sint64),
                "enum" => ProtoTypeInfo(ProtoAlternativeType::Enum),
                _ => panic!("Unknown proto type!: {}", ident_string),
            })
        }
    }
}

struct OneofFieldIdents(Vec<Ident>);

impl Parse for OneofFieldIdents {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parsed = Punctuated::<Ident, Token![,]>::parse_terminated(input).unwrap();
        Ok(OneofFieldIdents(parsed.into_iter().collect()))
    }
}