use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
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
        (false, true) => get_enum_derives(name, starting_tag, unit_fields),
        (false, false) => panic!("You can only form an enum consisting of either all simple variants or all typed variants (making a oneof type) whereas both types were specified here - please go for just one!")
    }
}

fn get_enum_derives(name: &Ident, starting_tag: u8, unit_fields: Vec<Ident>) -> TokenStream {
    let enum_values = (1..=(unit_fields.len() as i32)).into_iter().collect::<Vec<_>>();
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

            fn get_from_parquet_row(row: parquet::record::Row) -> (Self, u64) where Self: Sized {
                unreachable!(concat!("fn \"get_from_parquet_row\" should never be called for an enum type! Enum type: ", stringify!(#name)));
            }
        }

        impl derives::ProtoInfo for #name {
            fn get_proto_field_info(field_name: String, field_number: u8) -> derives::proto_structure_info::FieldInfo {
                derives::proto_structure_info::FieldInfo {
                    field_name,
                    field_type: derives::proto_structure_info::FieldType::Enum(derives::proto_structure_info::EnumInfo::from_fields(#starting_tag, #enum_fields_vec)),
                    field_specification: derives::proto_structure_info::FieldSpecification::Required,
                    field_number: field_number as u64,
                }
            }

            fn get_proto_structure_info() -> derives::proto_structure_info::MessageInfo {
                unreachable!(concat!("fn \"get_proto_structure_info\" should never be called for an enum type! Enum type: ", stringify!(#name)));
            }
        }

        impl derives::GenRandSamples for #name {
            fn get_sample<R: rand::Rng>(rng: &mut R) -> Self where Self: Sized {
                let enum_value = rng.gen_range(1..=#num_variants);
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

fn get_oneof_derives(name: &Ident, starting_tag: u8, unnamed_fields: Vec<(Ident, Type)>) -> TokenStream {
    quote! {

    }
}
