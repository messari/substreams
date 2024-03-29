use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
// use futures::StreamExt;
use parquet::basic::Repetition;
use prost_types::field_descriptor_proto::{Label, Type};
use prost_types::{DescriptorProto, EnumDescriptorProto, FieldDescriptorProto, FileDescriptorProto};
use std::borrow::BorrowMut;

/// Gives a cleaned up representation of all info need to construct the output proto type for a given substream module
#[derive(PartialEq, Clone, Debug)]
pub struct MessageInfo {
    pub type_name: String,
    pub field_specification: FieldSpecification,
    pub fields: Vec<FieldInfo>,
    pub oneof_groups: Vec<Vec<u64>>
}

impl MessageInfo {
    pub fn new(proto_descriptors: &Vec<FileDescriptorProto>, proto_type_name: &str, field_specification: FieldSpecification, parent_message_descriptor: Option<&DescriptorProto>) -> Self {
        let message = get_proto_type(proto_descriptors, proto_type_name, parent_message_descriptor);

        let mut oneof_group_mappings: HashMap<u64, HashSet<u64>> = HashMap::new();
        let mut fields = Vec::new();
        for field in &message.field {
            let field_specification = field.get_field_specification();
            let field_number = field.get_field_number();
            if let Some(index) = field.get_oneof_index() {
                if oneof_group_mappings.contains_key(&index) {
                    oneof_group_mappings.get_mut(&index).unwrap().insert(field_number);
                } else {
                    let mut field_numbers = HashSet::new();
                    field_numbers.insert(field_number);
                    oneof_group_mappings.insert(index, field_numbers);
                }
            }
            fields.push(FieldInfo {
                field_name: field.name().to_string(),
                field_type: field.get_field_type(proto_descriptors, &field_specification, message),
                field_specification,
                field_number,
            });
        }

        let oneof_groups = oneof_group_mappings.into_values().filter_map(|group| {
            if group.len() > 1 {
                Some(group.into_iter().collect())
            } else {
                // "Oneof groups of size one aren't actually oneof groups - they are just optional fields so we won't count these
                None
            }
        }).collect::<Vec<_>>();

        let oneof_fields = oneof_groups.clone().into_iter().flat_map(|x| x).collect::<Vec<_>>();
        for field in fields.iter_mut() {
            if oneof_fields.contains(&field.field_number) {
                assert_eq!(field.field_specification, FieldSpecification::Required, "All oneof fields should be required! TODO: Flesh out this error");
                field.set_as_optional_field(); // Once the oneof is taken to account each field inside will become optional so this needs to be taken into account
            }
        }

        MessageInfo {
            type_name: message.name().to_string(), // TODO: Should probably make sure this isn't a "full path type" and instead just the actual type name as specified in the proto
            field_specification,
            fields,
            oneof_groups,
        }
    }

    pub fn assert_block_number_field_not_manually_specified(&self) {
        for field in self.fields.iter() {
            assert_ne!(field.field_name, "block_number", "Can't specify \"block_number\" for a top hierachy field as we are already manually adding this in field in the sink - please remove this!");
        }
    }

    pub fn is_collection_of_items(&self) -> bool {
        if self.fields.len()==1 {
            let inner_field = self.fields[0].borrow();
            if inner_field.field_specification == FieldSpecification::Repeated &&
                inner_field.is_struct_field() {
                return true;
            }
        }
        false
    }

    pub fn is_oneof_type(&self) -> bool {
        if self.oneof_groups.len()==1 && self.oneof_groups[0].len()==self.fields.len() {
            for field in self.fields.iter() {
                if !field.is_struct_field() {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }

    pub fn get_item_type_info(mut self) -> (MessageInfo, u64) {
        let field = self.fields.pop().unwrap();
        let field_number = field.field_number;
        (field.get_struct_info().0, field_number)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct EnumInfo {
    // Key is the field number, and value is the corresponding enum value
    enum_mappings: HashMap<u64, String>
}

impl EnumInfo {
    pub fn new(proto_descriptors: &Vec<FileDescriptorProto>, proto_type_name: &str, parent_message_descriptor: &DescriptorProto) -> Self {
        let enum_type = get_enum_type(proto_descriptors, proto_type_name, parent_message_descriptor);

        EnumInfo {
            enum_mappings: enum_type.value.iter().map(|enum_value| (enum_value.number.unwrap() as u64, enum_value.name().to_string())).collect(),
        }
    }

    pub fn from_fields(mut starting_tag: u8, field_names: Vec<String>) -> Self {
        let mut enum_mappings = HashMap::new();

        for field_name in field_names {
            enum_mappings.insert(starting_tag as u64, field_name);
            starting_tag += 1;
        }

        EnumInfo {
            enum_mappings,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct FieldInfo {
    pub field_name: String,
    pub field_type: FieldType,
    pub field_specification: FieldSpecification,
    pub field_number: u64
}

impl FieldInfo {
    pub fn is_struct_field(&self) -> bool {
        if let FieldType::Message(_) = self.field_type.borrow() {
            true
        } else {
            false
        }
    }

    pub fn is_enum_field(&self) -> bool {
        if let FieldType::Enum(_) = self.field_type.borrow() {
            true
        } else {
            false
        }
    }

    pub fn get_struct_info(self) -> (MessageInfo, String) {
        match self.field_type {
            FieldType::Message(message_info) => (message_info, self.field_name),
            _ => panic!("No message info found! TODO: Flesh out this error some more")
        }
    }

    pub fn get_enum_mappings(&self) -> &HashMap<u64, String> {
        if let FieldType::Enum(enum_info) = &self.field_type {
            &enum_info.enum_mappings
        } else {
            unreachable!()
        }
    }

    pub fn set_as_optional_field(&mut self) {
        self.field_specification = FieldSpecification::Optional;
        if let FieldType::Message(message_info) = self.field_type.borrow_mut() {
            message_info.field_specification = FieldSpecification::Optional;
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum FieldType {
    Double,
    Float,
    Int64,
    Uint64,
    Int32,
    Fixed64,
    Fixed32,
    Bool,
    String,
    Message(MessageInfo),
    Bytes,
    Uint32,
    Enum(EnumInfo),
    Sfixed32,
    Sfixed64,
    Sint32,
    Sint64
}

pub fn get_proto_type<'a>(proto_files: &'a Vec<FileDescriptorProto>, proto_type: &str, parent_message_descriptor: Option<&'a DescriptorProto>) -> &'a DescriptorProto {
    let message_type = proto_type.split('.').last().unwrap().to_string();

    // First check to see if type is internally declared before checking all proto file descriptors
    if let Some(parent_message_descriptor) = parent_message_descriptor {
        for proto_message in parent_message_descriptor.nested_type.iter() {
            if proto_message.name.as_ref().unwrap() == &message_type {
                return proto_message;
            }
        }
    }

    // If not internally declared it should be declared in the root of one of the attached file descriptors
    for proto in proto_files.iter() {
        if proto_type.contains(proto.package.as_ref().unwrap()) {
            for proto_message in proto.message_type.iter() {
                if proto_message.name.as_ref().unwrap() == &message_type {
                    return proto_message;
                }
            }
        }
    }

    panic!("TODO: Something like: Unable to find proto type!!");
}

pub fn get_enum_type<'a>(proto_files: &'a Vec<FileDescriptorProto>, proto_type: &str, parent_message_descriptor: &'a DescriptorProto) -> &'a EnumDescriptorProto {
    let message_type = proto_type.split('.').last().unwrap().to_string();

    // First check to see if enum is internally declared before checking all proto file descriptors
    for proto_enum in parent_message_descriptor.enum_type.iter() {
        if proto_enum.name.as_ref().unwrap() == &message_type {
            return proto_enum;
        }
    }

    // If not internally declared it should be declared in the root of one of the attached file descriptors
    for proto in proto_files.iter() {
        if proto_type.contains(proto.package.as_ref().unwrap()) {
            for proto_enum in proto.enum_type.iter() {
                if proto_enum.name.as_ref().unwrap() == &message_type {
                    return proto_enum;
                }
            }
        }
    }

    panic!("TODO: Something like: Unable to find proto enum!!");
}

trait ProtoFieldExt {
    fn get_field_specification(&self) -> FieldSpecification;
    fn get_field_type(&self, proto_descriptors: &Vec<FileDescriptorProto>, field_specification: &FieldSpecification, parent_message_descriptor: &DescriptorProto) -> FieldType;
    fn get_field_number(&self) -> u64;
    fn get_oneof_index(&self) -> Option<u64>;
}

impl ProtoFieldExt for FieldDescriptorProto {
    fn get_field_specification(&self) -> FieldSpecification {
        // TODO: Check to see if Label::Optional field is reliable. If so then we can remove this check
        if self.proto3_optional == Some(true) {
            return FieldSpecification::Optional;
        }

        match self.label.unwrap() {
            x if x==(Label::Optional as i32) => {
                if self.proto3_optional == Some(true) {
                    FieldSpecification::Optional
                } else {
                    FieldSpecification::Required
                }
            },
            x if x==(Label::Required as i32) => FieldSpecification::Required,
            x if x==(Label::Repeated as i32) => {
                const SUPPORTED_REPEATED_TYPES: [i32; 13] = [Type::Double as i32, Type::Float as i32, Type::Int64 as i32, Type::Uint64 as i32, Type::Int32 as i32, Type::Fixed64 as i32,
                    Type::Fixed32 as i32, Type::Bool as i32, Type::Uint32 as i32, Type::Sfixed32 as i32, Type::Sfixed64 as i32, Type::Sint32 as i32, Type::Sint64 as i32];

                if SUPPORTED_REPEATED_TYPES.contains(&self.r#type.unwrap()) {
                    // TODO: In order to guarantee that the field is actually package we also need to make sure that they haven't added
                    // TODO: a packed==true/false flag to the field type declaration (So we should also be checking for this)
                    FieldSpecification::Packed
                } else {
                    FieldSpecification::Repeated
                }
            },
            _ => unreachable!()
        }
    }

    fn get_field_type(&self, proto_descriptors: &Vec<FileDescriptorProto>, field_specification: &FieldSpecification, parent_message_descriptor: &DescriptorProto) -> FieldType {
        match self.r#type.unwrap() {
            x if x == (Type::Double as i32) => FieldType::Double,
            x if x == (Type::Float as i32) => FieldType::Float,
            x if x == (Type::Int64 as i32) => FieldType::Int64,
            x if x == (Type::Uint64 as i32) => FieldType::Uint64,
            x if x == (Type::Int32 as i32) => FieldType::Int32,
            x if x == (Type::Fixed64 as i32) => FieldType::Fixed64,
            x if x == (Type::Fixed32 as i32) => FieldType::Fixed32,
            x if x == (Type::Bool as i32) => FieldType::Bool,
            x if x == (Type::String as i32) => FieldType::String,
            x if x == (Type::Message as i32) => {
                FieldType::Message(MessageInfo::new(proto_descriptors, self.type_name(), field_specification.clone(), Some(parent_message_descriptor)))
            },
            x if x == (Type::Bytes as i32) => FieldType::Bytes,
            x if x == (Type::Uint32 as i32) => FieldType::Uint32,
            x if x == (Type::Enum as i32) => {
                FieldType::Enum(EnumInfo::new(proto_descriptors, self.type_name(), parent_message_descriptor))
            },
            x if x == (Type::Sfixed32 as i32) => FieldType::Sfixed32,
            x if x == (Type::Sfixed64 as i32) => FieldType::Sfixed64,
            x if x == (Type::Sint32 as i32) => FieldType::Sint32,
            x if x == (Type::Sint64 as i32) => FieldType::Sint64,
            _ => unreachable!()
        }
    }

    fn get_field_number(&self) -> u64 {
        self.number() as u64
    }

    fn get_oneof_index(&self) -> Option<u64> {
        self.oneof_index.map(|index| index as u64)
    }
}


#[derive(PartialEq, Clone, Debug)]
pub enum FieldSpecification {
    Required,
    Optional,
    Repeated,
    Packed
}

impl FieldSpecification {
    pub fn get_repetition(&self) -> Repetition {
        match self {
            FieldSpecification::Required => Repetition::REQUIRED,
            FieldSpecification::Optional => Repetition::OPTIONAL,
            FieldSpecification::Repeated => Repetition::REPEATED,
            FieldSpecification::Packed => Repetition::REPEATED
        }
    }

    pub fn is_optional_or_required(&self) -> bool {
        match self {
            FieldSpecification::Required => true,
            FieldSpecification::Optional => true,
            _ => false
        }
    }
}