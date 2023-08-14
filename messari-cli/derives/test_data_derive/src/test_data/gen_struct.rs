use std::fmt::Debug;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, format_ident};
use syn::{DataStruct, Fields, Type};
use quote::ToTokens;
use crate::test_data::proto_alternative_type::{parse_proto_alternate_type, ProtoAlternativeType};

pub fn generate(name: &Ident, data_struct: DataStruct, starting_tag: u8) -> TokenStream {
    let field_info = parse_field_info(data_struct, starting_tag);

    let name_dto = format_ident!("{}Dto", name);
    let proto_field_attributes = field_info.proto_field_attributes;
    let field_value_initialisations = field_info.field_value_initialisations;
    let field_names = field_info.field_names;
    let proto_field_types = field_info.proto_field_types;
    let field_types = field_info.field_types;

    let oneof_groups_initialisation = get_oneof_groups_initialisation(&field_info.field_associations, starting_tag);

    let struct_and_oneof_match_statements = if field_info.field_associations.len() == 1 && field_info.field_associations.first().unwrap().is_not_oneof_field() {
        field_info.field_associations.first().unwrap().get_match_statements_for_single_field_struct()
    } else {
        field_info.field_associations.into_iter().flat_map(|field_assocation| {
            field_assocation.get_match_statements()
        }).collect::<Vec<_>>()
    };

    quote! {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct #name_dto {
            #(#proto_field_attributes
            pub #field_names: #proto_field_types,
            )*
        }

        impl TestData for #name {
            type ProtoType = #name_dto;

            fn to_proto_bytes(&self) -> Vec<u8> {
                let proto_struct = #name_dto {
                    #(#field_names: #field_value_initialisations,
                    )*
                };

                use prost::Message as _Message;
                proto_struct.encode_to_vec()
            }

            fn get_proto_value(&self) -> Self::ProtoType {
                #name_dto {
                    #(#field_names: #field_value_initialisations,
                    )*
                }
            }

            fn get_from_parquet_row<'a, T: Iterator<Item=(&'a String, &'a parquet::record::Field)>>(row: T) -> (Self, Option<u64>) where Self: Sized {
                #(let mut #field_names = None;
                )*
                let mut block_number = None;
                for (field_name, field_value) in row {
                    match field_name.as_str() {
                        #(#struct_and_oneof_match_statements
                        )*
                        "block_number" => {
                            if let parquet::record::Field::ULong(block_num) = field_value {
                                block_number = Some(block_num.clone());
                            } else {
                                panic!("Block number field not of type u64!\nField_value: {:?}", field_value);
                            }
                        },
                        _ => {
                            panic!(concat!("{} is not a valid field name for this struct. Struct type: ", stringify!(#name)), field_name);
                        }
                    }
                }

                (Self {
                    #(#field_names: #field_names.expect(concat!("Field: ", stringify!(#field_names), ", was not seen in parquet row!")),
                    )*
                }, block_number)
            }
        }

        impl derives::ProtoInfo for #name {
            fn add_proto_field_info(field_name: String, field_number: &mut u8, fields: &mut Vec<derives::proto_structure_info::FieldInfo>) {
                fields.push(derives::proto_structure_info::FieldInfo {
                    field_name,
                    field_type: derives::proto_structure_info::FieldType::Message(Self::get_proto_structure_info()),
                    field_specification: derives::proto_structure_info::FieldSpecification::Required,
                    field_number: *field_number as u64,
                });

                *field_number += 1;
            }

            fn get_proto_structure_info() -> derives::proto_structure_info::MessageInfo {
                let mut fields = Vec::new();
                let mut field_number = 1;
                #(#field_types::add_proto_field_info(stringify!(#field_names).to_string(), &mut field_number, &mut fields);
                )*

                derives::proto_structure_info::MessageInfo {
                    type_name: stringify!(#name).to_string(),
                    field_specification: derives::proto_structure_info::FieldSpecification::Required, // Get's overriden by parent struct later if a subfield to another type
                    fields,
                    oneof_groups: #oneof_groups_initialisation
                }
            }
        }

        impl derives::GenRandSamples for #name {
            fn get_sample<R: rand::Rng>(rng: &mut R) -> Self where Self: Sized {
                Self {
                    #(#field_names: #field_types::get_sample(rng),
                    )*
                }
            }
        }

        impl Default for #name {
            fn default() -> Self {
                Self {
                    #(#field_names: Default::default(),
                    )*
                }
            }
        }

        impl Clone for #name {
            fn clone(&self) -> Self {
                Self {
                    #(#field_names: self.#field_names.clone(),
                    )*
                }
            }
        }

        impl PartialEq for #name {
            fn eq(&self, other: &Self) -> bool {
                let mut equal = true;
                #(if !self.#field_names.eq(&other.#field_names) {
                    equal = false;
                })*
                equal
            }
        }

        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!(#name))
                 #(.field(stringify!(#field_names), &self.#field_names)
                 )*
                 .finish()
            }
        }
    }
}

fn get_oneof_groups_initialisation(field_associations: &Vec<FieldAssociation>, mut tag_number: u8) -> TokenStream {
    let mut oneof_group_array_initialisations = Vec::new();
    for field_association in field_associations.iter() {
        match field_association {
            FieldAssociation::OneofField { oneof_fields, .. } => {
                let mut field_nums = Vec::new();
                let first_field_number = tag_number as u64;
                tag_number += 1;
                for _ in 0..oneof_fields.len()-1 {
                    field_nums.push(tag_number as u64);
                    tag_number += 1;
                }
                oneof_group_array_initialisations.push(quote!(vec![#first_field_number#(, #field_nums)*]));
            }
            FieldAssociation::FieldName(_) => {
                tag_number += 1;
            }
        }
    }

    if oneof_group_array_initialisations.is_empty() {
        quote!(vec![])
    } else {
        let mut oneof_group_array_initialisations_iter = oneof_group_array_initialisations.into_iter();
        let first_group_initialisation = oneof_group_array_initialisations_iter.next().unwrap();
        let group_initialisations = oneof_group_array_initialisations_iter.collect::<Vec<_>>();

        quote!(vec![#first_group_initialisation, #(, #group_initialisations)*])
    }
}

#[derive(Debug)]
enum RepetitionType {
    Required,
    Optional,
    Repeated
}

impl RepetitionType {
    fn get_repetition_contribution(&self) -> &str {
        match self {
            RepetitionType::Required => "",
            RepetitionType::Optional => " optional,",
            RepetitionType::Repeated => " repeated,",
        }
    }
}

fn parse_field_info(data_struct: DataStruct, mut tag_number: u8) -> FieldInfo {
    let fields_named = match data_struct.fields {
        Fields::Named(fields_named) => {
            fields_named
        }
        Fields::Unnamed(_) => {
            panic!("Struct tuples are not allowed when creating test data!!!");
        }
        Fields::Unit => {
            panic!("Unit struct not allowed! Please add some fields to your test data struct!!");
        },
    };

    let mut field_info = FieldInfo::default();
    for field in fields_named.named.into_iter() {
        let proto_alternative_type = parse_proto_alternate_type(&field);

        let (type_string, outer_type) = extract_type(&field.ty);

        check_type(&type_string, &proto_alternative_type);

        let (is_oneof_type, is_enum_type) = if let Some(proto_type) = proto_alternative_type.as_ref() {
            (proto_type.is_oneof_type(), proto_type.is_enum_type())
        } else {
            (false, false)
        };

        let is_struct_type = if is_oneof_type || is_enum_type {
            false
        } else {
            is_struct_type(&type_string)
        };

        let (proto_field_type, option_added) = get_proto_field_type(&type_string, &outer_type, is_oneof_type, is_struct_type, is_enum_type);

        let repetition_type = get_repetition_type(&type_string);

        let field_ident = field.ident.unwrap();

        let field_value_initialisation = if option_added {
            quote!{ Some(self.#field_ident.get_proto_value()) }
        } else {
            quote!{ self.#field_ident.get_proto_value() }
        };

        let inner_type = get_inner_type(&type_string);
        let proto_type = get_proto_type(&proto_alternative_type, is_struct_type, &inner_type);
        let tag_info = get_tag_info(&mut tag_number, &proto_alternative_type);

        let proto_field_attribute = format!("#[prost({},{}{})]", proto_type, repetition_type.get_repetition_contribution(), tag_info);

        let field_association = if proto_alternative_type.is_some() {
            if let Some(field_association) = proto_alternative_type.as_ref().unwrap().get_field_assocation(field_ident.to_string(), inner_type) {
                field_association
            } else {
                let parquet_type = get_parquet_type(&type_string, &proto_alternative_type, is_struct_type);

                FieldAssociation::from_field_info(BasicFieldInfo {
                    field_name: field_ident.to_string(),
                    field_type: parquet_type,
                    repetition_type
                })
            }
        } else {
            let parquet_type = get_parquet_type(&type_string, &proto_alternative_type, is_struct_type);

            FieldAssociation::from_field_info(BasicFieldInfo {
                field_name: field_ident.to_string(),
                field_type: parquet_type,
                repetition_type
            })
        };

        field_info.add_field_info(field_ident, field.ty, field_value_initialisation, proto_field_type, proto_field_attribute, field_association);
    }

    field_info
}



fn get_parquet_type(type_string: &str, proto_alternative_type: &Option<ProtoAlternativeType>, is_struct: bool) -> ParquetType {
    let inner_type = get_inner_type(type_string);

    if is_struct {
        return ParquetType::Struct(inner_type);
    }

    if let Some(proto_alternative_type) = proto_alternative_type {
        proto_alternative_type.get_parquet_type(&inner_type)
    } else {
        match inner_type.as_str() {
            "bool" => ParquetType::Bool,
            "i32" => ParquetType::Int,
            "i64" => ParquetType::Long,
            "u32" => ParquetType::UInt,
            "u64" => ParquetType::ULong,
            "f32" => ParquetType::Float,
            "f64" => ParquetType::Double,
            "Vec<u8>" => ParquetType::Bytes,
            "String" => ParquetType::String,
            _ => unreachable!()
        }
    }
}

#[derive(Debug)]
pub(crate) enum ParquetType {
    Bool,
    Int,
    Long,
    UInt,
    ULong,
    Float,
    Double,
    String,
    Bytes,
    Enum(String), // String represents the enum type needed for reconstructing the enum from the given parquet string value
    Struct(String) // String represents the name of the struct type
}

impl ParquetType {
    pub(crate) fn get_unwrap_statement(&self) -> String {
        // TODO: There should be an indentation amount arg in order to offset the right amount based on the given repetition type
        macro_rules! val_unwrap {
            ($value_type:ident) => {
                concat!("let field_val = if let parquet::record::Field::",
                stringify!($value_type),
                "(field_val) = field_value {field_val.clone()} else {unreachable!(\"Parquet type read does not match expected type. Expected: ",
                stringify!($value_type),
                "actual: {:?}\", field_value)};").to_string()
            }
        }


        match self {
            ParquetType::Bool => val_unwrap!(Bool),
            ParquetType::Int => val_unwrap!(Int),
            ParquetType::Long => val_unwrap!(Long),
            ParquetType::UInt => val_unwrap!(UInt),
            ParquetType::ULong => val_unwrap!(ULong),
            ParquetType::Float => val_unwrap!(Float),
            ParquetType::Double => val_unwrap!(Double),
            ParquetType::String => val_unwrap!(Str),
            ParquetType::Bytes => val_unwrap!(Bytes),
            ParquetType::Enum(enum_type) => format!("let field_val: {0} = if let parquet::record::Field::Str(field_val) = field_value {{\n        \
                                                                    field_val.into()\n    \
                                                                }} else {{\n        \
                                                                    unreachable!(\"Parquet type read does not match expected type. Expected: Str for deriving enum type: {0}, actual: {{:?}}\", field_value)\n    \
                                                                }};", enum_type),
            ParquetType::Struct(struct_type) => format!("let field_val = if let parquet::record::Field::ListInternal(list_internal) = field_value {{\n        \
                                                                    if list_internal.len() == 1 {{\n            \
                                                                        let inner_unwrapped_list_element = list_internal.elements().iter().next().unwrap();
                                                                        if let parquet::record::Field::Group(field_val) = inner_unwrapped_list_element {{\n        \
                                                                            {0}::get_from_parquet_row(field_val.get_column_iter()).0\n    \
                                                                        }} else {{\n        \
                                                                            {0}::get_from_parquet_row(vec![(&\"single_field\".to_string(), inner_unwrapped_list_element)].into_iter()).0\n    \
                                                                        }}
                                                                    }}\n    \
                                                                    else {{
                                                                        {0}::get_from_parquet_row(vec![(&\"single_field\".to_string(), field_value)].into_iter()).0
                                                                    }}
                                                                 }} else {{
                                                                    if let parquet::record::Field::Group(field_val) = field_value {{\n        \
                                                                        {0}::get_from_parquet_row(field_val.get_column_iter()).0\n    \
                                                                    }} else {{\n        \
                                                                        {0}::get_from_parquet_row(vec![(&\"single_field\".to_string(), field_value)].into_iter()).0\n    \
                                                                    }}
                                                                 }};", struct_type),
        }
    }
}

fn get_tag_info(tag_number: &mut u8, proto_alternative_type: &Option<ProtoAlternativeType>) -> String {
    let num_tags = if let Some(proto_alternative_type) = proto_alternative_type {
        proto_alternative_type.get_num_tags()
    } else {
        1
    };

    if num_tags==1 {
        let tag_info = format!("tag=\"{}\"", tag_number);
        *tag_number += 1;
        tag_info
    } else {
        let mut tag_info = format!("tags=\"{}", tag_number);
        *tag_number += 1;
        for _ in 1..num_tags {
            tag_info.push_str(&format!(", {}", tag_number));
            *tag_number += 1;
        }
        tag_info.push('"');
        tag_info
    }
}

fn get_proto_type(proto_alternative_type: &Option<ProtoAlternativeType>, is_struct_type: bool, inner_type: &str) -> String {
    if let Some(proto_type) = proto_alternative_type {
        proto_type.get_proto_type(inner_type)
    } else {
        if is_struct_type {
            "message".to_string()
        } else {
            match inner_type {
                "Vec<u8>" => "bytes=\"vec\"".to_string(),
                "f32" => "float".to_string(),
                "f64" => "double".to_string(),
                "i32" => "int32".to_string(),
                "i64" => "int64".to_string(),
                "u32" => "uint32".to_string(),
                "u64" => "uint64".to_string(),
                "bool" => "bool".to_string(),
                "String" => "string".to_string(),
                _ => unreachable!()
            }
        }
    }
}

/// Returns tuple in form => (proto_field_type, option_added)
fn get_proto_field_type(type_string: &str, outer_type: &str, is_oneof_type: bool, is_struct_type: bool, is_enum_type: bool) -> (String, bool) {
    if is_enum_type {
        return ("i32".to_string(), false);
    }

    let mut proto_field_type = if is_struct_type || is_oneof_type {
        let inner_type = get_inner_type(type_string);
        let type_name = inner_type.split('<').next().unwrap().to_string();
        let type_name_dto = format!("{}Dto", type_name);
        type_string.replace(&type_name, &type_name_dto)
    } else {
        type_string.to_string()
    };
    proto_field_type = proto_field_type.replace("Vec<", "::prost::alloc::vec::Vec<");
    proto_field_type = proto_field_type.replace("Option<", "::core::option::Option<");
    proto_field_type = proto_field_type.replace("String", "::prost::alloc::string::String"); // TODO: Make this safe like the above two replacements

    let option_added = if is_oneof_type || (is_struct_type && outer_type!="Option" && outer_type!="Vec") {
        proto_field_type = "::core::option::Option<".to_string() + &proto_field_type + ">";
        true
    } else {
        false
    };

    (proto_field_type, option_added)
}

fn get_repetition_type(type_string: &str) -> RepetitionType {
    if type_string.starts_with("Option<") {
        return RepetitionType::Optional;
    }

    if type_string.starts_with("Vec<") {
        if type_string != "Vec<u8>" {
            return RepetitionType::Repeated;
        }
    }

    RepetitionType::Required
}

fn is_struct_type(type_string: &str) -> bool {
    const NON_STRUCT_TYPES: [&str; 9] = ["Vec<u8>", "f32", "f64", "i32", "i64", "u32", "u64", "bool", "String"];
    let inner_type = get_inner_type(type_string);
    !NON_STRUCT_TYPES.contains(&inner_type.as_str())
}

/// Assumes type is either of form OuterType<InnerType> or InnerType.
fn get_inner_type(type_string: &str) -> String {
    if type_string.starts_with("Option<") {
        return type_string[7..type_string.len()-1].to_string();
    }

    if type_string.starts_with("Vec<") {
        if type_string != "Vec<u8>" {
            return type_string[4..type_string.len()-1].to_string();
        }
    }

    type_string.to_string()
}

#[derive(Default)]
struct FieldInfo {
    field_names: Vec<TokenStream>,
    field_types: Vec<TokenStream>,
    field_value_initialisations: Vec<TokenStream>,
    proto_field_types: Vec<TokenStream>,
    proto_field_attributes: Vec<TokenStream>,
    field_associations: Vec<FieldAssociation>,
}

impl FieldInfo {
    fn add_field_info(&mut self, field_ident: Ident, field_type: Type, field_value_initialisation: TokenStream, proto_field_type: String, proto_field_attribute: String, field_association: FieldAssociation) {
        self.field_names.push(field_ident.to_token_stream());
        let field_type_string = field_type.to_token_stream().to_string();
        if field_type_string.contains("<") {
            self.field_types.push(field_type_string.replace("<", "::<").parse().unwrap())
        } else {
            self.field_types.push(field_type.to_token_stream());
        }
        self.field_value_initialisations.push(field_value_initialisation);
        self.proto_field_types.push(proto_field_type.parse().unwrap());
        self.proto_field_attributes.push(proto_field_attribute.parse().unwrap());
        self.field_associations.push(field_association);
    }
}
#[derive(Debug)]
pub(crate) struct BasicFieldInfo {
    field_name: String,
    field_type: ParquetType,
    repetition_type: RepetitionType
}

impl BasicFieldInfo {
    fn get_unwrap_statement(&self) -> String {
        match self.repetition_type {
            RepetitionType::Required => {
                format!("{}", self.field_type.get_unwrap_statement())
            },
            RepetitionType::Optional => {
                format!("let field_val = if field_value == &parquet::record::Field::Null {{\n            \
                                    None\n        \
                                }} else {{\n            \
                                    {}\n            \
                                    Some(field_val)\n        \
                                }};",
                        self.field_type.get_unwrap_statement())
            },
            RepetitionType::Repeated => {
                format!("let field_val = if let parquet::record::Field::ListInternal(list_internal) = field_value {{\n            \
                                    let mut parsed_values = Vec::new();
                                    let mut list_unwrapped = list_internal;
                                    if list_internal.len() == 1 {{\n            \
                                        if let parquet::record::Field::ListInternal(double_list_internal) = list_internal.elements().iter().next().unwrap() {{
                                            list_unwrapped = double_list_internal;
                                        }}
                                    }}\n    \
                                    for field_value in list_unwrapped.elements() {{\n                \
                                        {0}\n                \
                                        parsed_values.push(field_val);\n            \
                                    }}\n        \
                                    parsed_values\n    \
                                }} else if field_value == &parquet::record::Field::Null {{\n       \
                                    Vec::new()
                                }} else {{\n       \
                                    panic!(\"Field is repeated although list type or null type was not returned! field_value: {{:?}}\", field_value)\n    \
                                }};",
                        self.field_type.get_unwrap_statement())
            },
        }
    }
}

#[derive(Debug)]
pub(crate) enum FieldAssociation {
    FieldName(BasicFieldInfo),
    OneofField {
        field_name: String,
        field_type: String,
        oneof_fields: Vec<BasicFieldInfo>
    }
}

impl FieldAssociation {
    pub(crate) fn from_field_info(field_info: BasicFieldInfo) -> Self {
        FieldAssociation::FieldName(field_info)
    }

    pub(crate) fn is_not_oneof_field(&self) -> bool {
        match self {
            FieldAssociation::FieldName(_) => true,
            _ => false
        }
    }

    pub(crate) fn from_oneof_field_and_type_idents(field_name: String, field_type: String, oneof_field_and_type_info: Vec<(String, String)>) -> Self {
        fn parse_parquet_type(type_ident: &str) -> ParquetType {
            match type_ident {
                "bool" => ParquetType::Bool,
                "i32" => ParquetType::Int,
                "i64" => ParquetType::Long,
                "u32" => ParquetType::UInt,
                "u64" => ParquetType::ULong,
                "f32" => ParquetType::Float,
                "f64" => ParquetType::Double,
                "Vec<u8>" => ParquetType::Bytes,
                "String" => ParquetType::String,
                _ => ParquetType::Struct(type_ident.to_string())
            }
        }

        FieldAssociation::OneofField {
            field_name,
            field_type,
            oneof_fields: oneof_field_and_type_info.into_iter().map(|(field_ident, type_ident)| {
                BasicFieldInfo {
                    field_name: field_ident,
                    field_type: parse_parquet_type(&type_ident),
                    repetition_type: RepetitionType::Optional,
                }
            }).collect(),
        }
    }

    pub(crate) fn get_match_statements_for_single_field_struct(&self) -> Vec<TokenStream> {
        if let FieldAssociation::FieldName(field_info) = self {
            let match_block = format!("\"single_field\" => {{\n    \
                                                                 {1}\n    \
                                                                 {0} = Some(field_val);\n\
                                                              }},", field_info.field_name, field_info.get_unwrap_statement());

            if field_info.field_name != "single_field" {
                let single_field_match_block = format!("\"{0}\" => {{\n    \
                                                                 {1}\n    \
                                                                 {0} = Some(field_val);\n\
                                                              }},", field_info.field_name, field_info.get_unwrap_statement());

                vec![match_block.parse().unwrap(), single_field_match_block.parse().unwrap()]
            } else {
                vec![match_block.parse().unwrap()]
            }
        } else {
            unreachable!()
        }
    }

    pub(crate) fn get_match_statements(&self) -> Vec<TokenStream> {
        match self {
            FieldAssociation::FieldName(field_info) => {
                let match_block = format!("\"{0}\" => {{\n    \
                                                                 {1}\n    \
                                                                 {0} = Some(field_val);\n\
                                                              }},", field_info.field_name, field_info.get_unwrap_statement());



                vec![match_block.parse().unwrap()]
            },
            FieldAssociation::OneofField {field_name, field_type, oneof_fields} => {
                oneof_fields.into_iter().map(|field_info| {
                    let match_block = format!("\"{3}\" => {{\n    \
                                                                {1}\n    \
                                                                if let Some(field_val) = field_val {{
                                                                    if {0}.is_some() {{\n    \
                                                                        panic!(\"There is more than one value set for oneof field: {0}!\");\n     \
                                                                    }} else {{\n        \
                                                                        {0} = Some({2}::{3}(field_val));\n    \
                                                                    }}
                                                                }}
                                                            }},", field_name, field_info.get_unwrap_statement(), field_type, field_info.field_name);

                    match_block.parse().unwrap()
                }).collect()
            }
        }
    }
}

fn check_type(type_string: &String, proto_alternative_type: &Option<ProtoAlternativeType>) {
    if type_string.contains('(') || type_string.contains(')') {
        panic!("Tuples not supported!\nType: {}", type_string);
    }

    if type_string.contains('[') || type_string.contains(']') {
        panic!("Arrays not supported!\nType: {}", type_string);
    }

    let mut outer_types = type_string.split('<').into_iter().collect::<Vec<_>>();
    if outer_types.len() <= 1 {
        const UNSUPPORTED_PRIMITIVE_TYPRS: [&str; 8] = ["i8", "i16", "i128", "isize", "u16", "u128", "usize", "char"];

        for unsupported_type in UNSUPPORTED_PRIMITIVE_TYPRS {
            if type_string.contains(unsupported_type) {
                panic!("Primitive type: {} is not compatible with proto message format!\nType: {}", unsupported_type, type_string);
            }
        }

        if let Some(proto_type) = proto_alternative_type {
            match proto_type {
                ProtoAlternativeType::Fixed64 => assert_eq!(type_string, "u64", "Proto alternative type: {:?} is not compatible with underlying field type!\nType: {}", proto_type, type_string),
                ProtoAlternativeType::Fixed32 => assert_eq!(type_string, "u32", "Proto alternative type: {:?} is not compatible with underlying field type!\nType: {}", proto_type, type_string),
                ProtoAlternativeType::Sfixed32 => assert_eq!(type_string, "i32", "Proto alternative type: {:?} is not compatible with underlying field type!\nType: {}", proto_type, type_string),
                ProtoAlternativeType::Sfixed64 => assert_eq!(type_string, "i64", "Proto alternative type: {:?} is not compatible with underlying field type!\nType: {}", proto_type, type_string),
                ProtoAlternativeType::Sint32 => assert_eq!(type_string, "i32", "Proto alternative type: {:?} is not compatible with underlying field type!\nType: {}", proto_type, type_string),
                ProtoAlternativeType::Sint64 => assert_eq!(type_string, "i64", "Proto alternative type: {:?} is not compatible with underlying field type!\nType: {}", proto_type, type_string),
                ProtoAlternativeType::Oneof(_) => {
                    assert!(!type_string.starts_with("Option<"), "Oneof type: {:?} can't be an optional type!\nType: {}", proto_type, type_string);
                    assert!(!type_string.starts_with("Vec<"), "Oneof type: {:?} can't be a repeated type!\nType: {}", proto_type, type_string);
                },
                _ => {}
            }
        }

        return;
    }

    outer_types.pop().unwrap();

    let option_count = outer_types.iter().filter(|&type_str| *type_str == "Option").count();
    let vec_count = outer_types.iter().filter(|&type_str| *type_str == "Vec").count();

    if option_count + vec_count > 1 {
        match (option_count>1, vec_count>1) {
            (true, true) => {
                panic!("Both multiple uses of Option and Vec types when declaring field! You should only being\
                 using just one occurrence of either of Option or Vec max!!\nType: {}", type_string)
            },
            (true, false) => {
                panic!("Multiple uses of Option type when declaring field! You should only being\
                 using just one occurrence of either of Option or Vec max!!\nType: {}", type_string)
            },
            (false, true) => {
                panic!("Multiple uses of Vec type when declaring field! You should only being\
                 using just one occurrence of either of Option or Vec max!!\nType: {}", type_string)
            },
            (false, false) => panic!("Both Option and Vec types were used when declaring field! You should only being\
                 using just one occurrence of either of Option or Vec max!!\nType: {}", type_string)
        }
    }
}

fn extract_type(t: &syn::Type) -> (String, String) {
    match t {
        syn::Type::Path(tp) => extract_type_path(tp),
        syn::Type::Reference(r) => extract_type(&r.elem),
        _ => panic!("This type is not supported: {:#?}", t),
    }
}

fn extract_type_path(tp: &syn::TypePath) -> (String, String) {
    let full_type = tp
        .to_token_stream()
        .to_string()
        .split_whitespace()
        .collect::<String>();
    let to_string = &tp.path.segments.last().unwrap().ident.to_string();

    (full_type, to_string.to_string())
}
