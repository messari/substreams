use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use futures::StreamExt;
use parquet::basic::Repetition;
use prost_types::field_descriptor_proto::{Label, Type};
use prost_types::{DescriptorProto, EnumDescriptorProto, FieldDescriptorProto, FileDescriptorProto};
use std::borrow::BorrowMut;
use derives::proto_structure_info::{FieldSpecification, MessageInfo};

use crate::streaming_fast::streamingfast_dtos::Package;

pub(crate) fn get_output_type_info(package: &Package, module_name: &str) -> (MessageInfo, String) {
    for module in package.modules.as_ref().unwrap().modules.iter() {
        if &module.name == module_name {
            let output_type = module.output.as_ref().unwrap().r#type.to_string();

            if !output_type.starts_with("proto:") {
                panic!("TODO!");
            }

            let message_info = MessageInfo::new(&package.proto_files, &output_type, FieldSpecification::Required);

            return (message_info, output_type);
        }
    }

    panic!("Couldn't find output type!!")
}
