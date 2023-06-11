use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens, format_ident};
use syn::{DataEnum, Fields, Type};

pub fn generate(name: &Ident, de: DataEnum, starting_tag: u8) -> TokenStream {
    let mut unit_fields = Vec::new();
    let mut unnamed_fields = Vec::new();
    for variant in de.variants.into_iter() {
        match variant.fields {
            Fields::Named(fields_named) => {
                panic!("Named fields are not allowed for enum variants when creating test data!!!\nNamed field: {}", variant.ident.to_string());
            }
            Fields::Unnamed(fields_unnamed) => {
                let mut fields_unnamed_iter = fields_unnamed.unnamed.into_iter();
                let unnamed_field = fields_unnamed_iter.next().expect("No field found in enum variant!");
                assert!(fields_unnamed_iter.next().is_none(), "More than one field type declared in enum variant!");
                unnamed_fields.push((variant.ident, unnamed_field.ty))
            }
            Fields::Unit => {
                unit_fields.push(variant.ident);
            },
        }
    }

    match (unit_fields.is_empty(), unnamed_fields.is_empty()) {
        (true, true) => panic!("Need to specify at least one field for enum!"),
        (true, false) => get_oneof_derives(name, starting_tag, unnamed_fields),
        (false, true) => get_enum_derives(name, unit_fields),
        (false, false) => panic!("You can only form an enum consisting of either all simple variants or all typed variants (making a oneof type) whereas both types were specified here - please go for just one!")
    }
}

fn get_enum_derives(name: &Ident, unit_fields: Vec<Ident>) -> TokenStream {
    let enum_values = (0..(unit_fields.len() as i32)).into_iter().collect::<Vec<_>>();
    let num_variants = unit_fields.len() as i32;

    let mut enum_fields_iter = unit_fields.clone().into_iter();
    let first_enum_field = enum_fields_iter.next().unwrap();
    let rest_of_the_enum_fields = enum_fields_iter.collect::<Vec<_>>();
    let enum_fields_vec = quote!{vec![stringify!(#first_enum_field).to_string()#(, stringify!(#rest_of_the_enum_fields).to_string())*]};

    quote! {
        impl TestData for #name {
            type ProtoType = i32;

            fn to_proto_bytes(&self) -> Vec<u8> {
                unreachable!(concat!("fn \"to_proto_bytes\" should never be called for an enum type! Enum type: ", stringify!(#name)));
            }

            fn get_proto_value(&self) -> Self::ProtoType {
                match self {
                    #(#name::#unit_fields => #enum_values,
                    )*
                }
            }

            fn get_from_parquet_row<'a, T: Iterator<Item=(&'a String, &'a parquet::record::Field)>>(row: T) -> (Self, Option<u64>) where Self: Sized {
                unreachable!(concat!("fn \"get_from_parquet_row\" should never be called for an enum type! Enum type: ", stringify!(#name)));
            }
        }

        impl derives::ProtoInfo for #name {
            fn add_proto_field_info(field_name: String, field_number: &mut u8, fields: &mut Vec<derives::proto_structure_info::FieldInfo>) {
                fields.push(derives::proto_structure_info::FieldInfo {
                    field_name,
                    field_type: derives::proto_structure_info::FieldType::Enum(derives::proto_structure_info::EnumInfo::from_fields(0, #enum_fields_vec)),
                    field_specification: derives::proto_structure_info::FieldSpecification::Required,
                    field_number: *field_number as u64,
                });

                *field_number += 1;
            }

            fn get_proto_structure_info() -> derives::proto_structure_info::MessageInfo {
                unreachable!(concat!("fn \"get_proto_structure_info\" should never be called for an enum type! Enum type: ", stringify!(#name)));
            }
        }

        impl derives::GenRandSamples for #name {
            fn get_sample<R: rand::Rng>(rng: &mut R) -> Self where Self: Sized {
                let enum_value = rng.gen_range(0..#num_variants);
                match enum_value {
                    #(#enum_values => #name::#unit_fields,
                    )*
                    _ => unreachable!()
                }
            }
        }

        impl From<&String> for #name {
            fn from(value: &String) -> Self {
                match value.as_str() {
                    #(stringify!(#unit_fields) => #name::#unit_fields,
                    )*
                    _ => panic!(concat!("String value: \"{}\" during conversion to type: ", stringify!(#name), " is not one of the enum's variants!"), value)
                }
            }
        }

        impl #name {
            fn from_i32(value: i32) -> Option<Self> {
                match value {
                    #(#enum_values => Some(#name::#unit_fields),
                    )*
                    _ => None
                }
            }
        }

        impl Default for #name {
            fn default() -> Self {
                #name::#first_enum_field
            }
        }

        impl Clone for #name {
            fn clone(&self) -> Self {
                match self {
                    #(#name::#unit_fields => #name::#unit_fields,
                    )*
                }
            }
        }

        impl PartialEq for #name {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    #((#name::#unit_fields, #name::#unit_fields) => true,
                    )*
                    _ => false
                }
            }
        }

        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#name::#unit_fields => write!(f, concat!(stringify!(#name), "::",stringify!(#unit_fields))),
                    )*
                }
            }
        }
    }
}

fn get_prost_attribute(variant_type: &Type, tag: u8) -> (TokenStream, bool) {
    let variant_type_string = variant_type.to_token_stream().to_string();
    assert!(!variant_type_string.starts_with("Option<"), "Can't have optional fields in enum! Variant type: {}", variant_type_string);
    assert!(!variant_type_string.starts_with("Vec<"), "Can't have repeated fields in enum! Variant type: {}", variant_type_string);

    match variant_type_string.as_str() {
        "Vec<u8>" => (format!("#[prost(bytes=\"vec\", tag=\"{}\")]", tag).parse().unwrap(), false),
        "f32" => (format!("#[prost(float, tag=\"{}\")]", tag).parse().unwrap(), false),
        "f64" => (format!("#[prost(double, tag=\"{}\")]", tag).parse().unwrap(), false),
        "i32" => (format!("#[prost(int32, tag=\"{}\")]", tag).parse().unwrap(), false),
        "i64" => (format!("#[prost(int64, tag=\"{}\")]", tag).parse().unwrap(), false),
        "u32" => (format!("#[prost(uint32, tag=\"{}\")]", tag).parse().unwrap(), false),
        "u64" => (format!("#[prost(uint64, tag=\"{}\")]", tag).parse().unwrap(), false),
        "bool" => (format!("#[prost(bool, tag=\"{}\")]", tag).parse().unwrap(), false),
        "String" => (format!("#[prost(string, tag=\"{}\")]", tag).parse().unwrap(), false),
        _ => (format!("#[prost(message, tag=\"{}\")]", tag).parse().unwrap(), true),
    }
}

fn get_oneof_derives(name: &Ident, mut starting_tag: u8, unnamed_fields: Vec<(Ident, Type)>) -> TokenStream {
    let enum_values = (0..(unnamed_fields.len() as i32)).into_iter().collect::<Vec<_>>();
    let num_variants = unnamed_fields.len() as i32;
    let name_dto = format_ident!("{}Dto", name);

    let (first_field, first_variant_type) = unnamed_fields.first().unwrap().clone();

    let mut proto_attributes = Vec::new();
    let mut proto_types = Vec::new();
    let mut variant_types = Vec::new();
    let mut fields = Vec::new();
    for (field, variant_type) in unnamed_fields.into_iter() {
        let (proto_attribute, is_struct) = get_prost_attribute(&variant_type, starting_tag);
        proto_attributes.push(proto_attribute);
        fields.push(field);
        if is_struct {
            proto_types.push(format_ident!("{}Dto", variant_type.to_token_stream().to_string()).to_token_stream());
        } else {
            proto_types.push(variant_type.to_token_stream());
        }
        variant_types.push(variant_type);
        starting_tag += 1;
    }

    quote! {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum #name_dto {
            #(#proto_attributes
            #fields(#proto_types),
            )*
        }

        impl TestData for #name {
            type ProtoType = #name_dto;

            fn to_proto_bytes(&self) -> Vec<u8> {
                unreachable!(concat!("fn \"to_proto_bytes\" should never be called for an enum type! Enum type: ", stringify!(#name)));
            }

            fn get_proto_value(&self) -> Self::ProtoType {
                match self {
                    #(#name::#fields(val) => #name_dto::#fields(val.get_proto_value()),
                    )*
                }
            }

            fn get_from_parquet_row<'a, T: Iterator<Item=(&'a String, &'a parquet::record::Field)>>(row: T) -> (Self, Option<u64>) where Self: Sized {
                unreachable!(concat!("fn \"get_from_parquet_row\" should never be called for an enum type! Enum type: ", stringify!(#name)));
            }
        }

        impl derives::ProtoInfo for #name {
            fn add_proto_field_info(field_name: String, field_number: &mut u8, fields: &mut Vec<derives::proto_structure_info::FieldInfo>) {
                #(#variant_types::add_proto_field_info(stringify!(#fields).to_string(), field_number, fields);
                    let field_info = fields.last_mut().unwrap();
                    field_info.field_specification = derives::proto_structure_info::FieldSpecification::Optional;
                    if let derives::proto_structure_info::FieldType::Message(ref mut message_info) = field_info.field_type {
                        message_info.field_specification = derives::proto_structure_info::FieldSpecification::Optional;
                    }
                )*
            }

            fn get_proto_structure_info() -> derives::proto_structure_info::MessageInfo {
                unreachable!(concat!("fn \"get_proto_structure_info\" should never be called for an enum type! Enum type: ", stringify!(#name)));
            }
        }

        impl derives::GenRandSamples for #name {
            fn get_sample<R: rand::Rng>(rng: &mut R) -> Self where Self: Sized {
                let enum_value = rng.gen_range(0..#num_variants);
                match enum_value {
                    #(#enum_values => #name::#fields(#variant_types::get_sample(rng)),
                    )*
                    _ => unreachable!()
                }
            }
        }

        impl Default for #name {
            fn default() -> Self {
                #name::#first_field(#first_variant_type::default())
            }
        }

        impl Clone for #name {
            fn clone(&self) -> Self {
                match self {
                    #(#name::#fields(val) => #name::#fields(val.clone()),
                    )*
                }
            }
        }

        impl PartialEq for #name {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    #((#name::#fields(val), #name::#fields(val2)) => val.eq(val2),
                    )*
                    _ => false
                }
            }
        }

        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#name::#fields(val) => write!(f, concat!(stringify!(#name), "::",stringify!(#fields), "({:?})"), val),
                    )*
                }
            }
        }
    }
}
