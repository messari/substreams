use derives::proto_structure_info::{FieldSpecification, MessageInfo};

use crate::streaming_fast::streamingfast_dtos::Package;

pub(crate) fn get_output_type_info(package: &Package, module_name: &str) -> (MessageInfo, String) {
    for module in package.modules.as_ref().unwrap().modules.iter() {
        if &module.name == module_name {
            let output_type = module.output.as_ref().expect(&format!("Module you are trying to process: {}, is not a map module!", module_name)).r#type.to_string();

            if !output_type.starts_with("proto:") {
                panic!("TODO!");
            }

            let message_info = MessageInfo::new(&package.proto_files, &output_type, FieldSpecification::Required, None);

            message_info.assert_block_number_field_not_manually_specified();

            return (message_info, output_type);
        }
    }

    panic!("Couldn't find output type!!")
}
