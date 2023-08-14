use syn::{Token, TypeTuple};
use proc_macro2::Ident;
use quote::ToTokens;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::punctuated::Punctuated;

use crate::test_data::gen_struct::{FieldAssociation, ParquetType};

pub(crate) fn parse_proto_alternate_type(field: &syn::Field) -> Option<ProtoAlternativeType> {
    let mut proto_alternative_types = field.attrs.iter().filter_map(|attribute| {
        if let Some(attribute_ident) = attribute.path().get_ident() {
            if attribute_ident == "proto_type" {
                return Some(attribute.parse_args::<ProtoTypeInfo>().expect(&format!("Unable to parse attribute info for proto_type! Attribute: {}", attribute.to_token_stream().to_string())).0);
            }
        }

        None
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
    Oneof(Vec<(String, String)>) // Each item in vec in form -> (field_name, field_type)
}

impl ProtoAlternativeType {
    pub(crate) fn is_oneof_type(&self) -> bool {
        if let ProtoAlternativeType::Oneof(_) = self {
            true
        } else {
            false
        }
    }

    pub(crate) fn is_enum_type(&self) -> bool {
        self == &ProtoAlternativeType::Enum
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

    pub(crate) fn get_field_assocation(&self, field_name: String, field_type: String) -> Option<FieldAssociation> {
        if let ProtoAlternativeType::Oneof(oneof_field_and_type_info) = self {
            Some(FieldAssociation::from_oneof_field_and_type_idents(field_name, field_type, oneof_field_and_type_info.clone()))
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
            ProtoAlternativeType::Oneof(_) => format!("oneof=\"{}Dto\"", inner_type),
        }
    }
}

struct ProtoTypeInfo(ProtoAlternativeType);

impl Parse for ProtoTypeInfo {
    fn parse(input: ParseStream) -> syn::Result<Self> {

        let ident: Ident = input.parse()?;

        let ident_string = ident.to_string();
        if ident_string=="Oneof" {
            let group = proc_macro2::Group::parse(input).unwrap();
            let field_idents = match syn::parse2::<OneofFieldInfo>(group.stream()) {
                Ok(oneof_field_info) => oneof_field_info.0,
                Err(_) => panic!("TODO@!!123")
            };

            if field_idents.len() == 0 {
                panic!("No fields were specified for oneof type! Please specify at least 2 field names for a oneof type.");
            }

            if field_idents.len() == 1 {
                panic!("Only 1 field was specified for oneof type! Please specify at least 2 field names for a oneof type. Field name specified: {}", field_idents.first().unwrap().0.to_string());
            }

            Ok(ProtoTypeInfo(ProtoAlternativeType::Oneof(field_idents)))
        } else {
            Ok(match ident_string.as_str() {
                "Fixed32" => ProtoTypeInfo(ProtoAlternativeType::Fixed32),
                "Fixed64" => ProtoTypeInfo(ProtoAlternativeType::Fixed64),
                "Sfixed32" => ProtoTypeInfo(ProtoAlternativeType::Sfixed32),
                "Sfixed64" => ProtoTypeInfo(ProtoAlternativeType::Sfixed64),
                "Sint32" => ProtoTypeInfo(ProtoAlternativeType::Sint32),
                "Sint64" => ProtoTypeInfo(ProtoAlternativeType::Sint64),
                "Enum" => ProtoTypeInfo(ProtoAlternativeType::Enum),
                _ => panic!("Unknown proto type!: {}", ident_string),
            })
        }
    }
}

struct OneofFieldInfo(Vec<(String, String)>); // In form: Vec<(field_name, field_type)

impl Parse for OneofFieldInfo {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parsed = Punctuated::<TypeTuple, Token![,]>::parse_terminated(input).unwrap();
        Ok(OneofFieldInfo(parsed.into_iter().map(|x| {
            let mut info_iterator = x.elems.into_iter().map(|x| x.to_token_stream().to_string());
            (info_iterator.next().unwrap(), info_iterator.next().unwrap())
        }).collect()))
    }
}
