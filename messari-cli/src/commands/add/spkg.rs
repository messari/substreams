use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::mem;
use clap::Parser;
use ethabi::{Contract, ParamType};
use serde_json::to_string;

use crate::abi::{add_abis, AbisArgs, AbiInfo, get_abi_file_contents};
use crate::project_dir::ProjectDirArg;
use crate::protocols::ProtocolAndNetworkArgs;
use crate::terminal_interface::{get_input, select_multiple_from_values};

#[derive(Parser)]
pub(crate) struct Spkg {
    #[clap(flatten)]
    pub(crate) project_dir: ProjectDirArg,
    #[clap(flatten)]
    pub(crate) protocol_and_network_args: ProtocolAndNetworkArgs,
    #[arg(
    short,
    long,
    value_name = "ABI",
    help = "ABI can be specified as local file path or as a contract address. Only one abi can be specified per spkg"
    )]
    pub(crate) abi: Option<String>,
    pub(crate) spkg_name: Option<String>,
}

impl Spkg {
    pub(crate) fn execute(&mut self) {
        let project_dir = self.project_dir.get_project_dir(true);
        let protocol_and_network_info = self.protocol_and_network_args.get_info();

        let abi_string = if let Some(abi) = &mut self.abi {
            mem::take(abi)
        } else {
            get_input("Abi", None, false)
        };

        let spkg_name = if let Some(spkg_name) = &mut self.spkg_name {
            mem::take(spkg_name)
        } else {
            get_input(".spkg name", None, false)
        };

        assert_spkg_name_allowed(&spkg_name);

        let abi_info: AbiInfo = abi_string.as_str().into();
        abi_info.assert_compatible_with_protocol(&protocol_and_network_info.protocol);

        let abi_file_contents = get_abi_file_contents(
            &abi_info,
            &protocol_and_network_info.protocol.protocol_type,
            &protocol_and_network_info.network,
        );

        let contract_info = Contract::load(abi_file_contents.as_bytes()).unwrap();

        println!("Contract: {:?}", contract_info);

        let mut events = parse_events(contract_info);

        if events.is_empty() {
            panic!("No events found for ABI! abi: {:?}\nabi_file_contents: {}", abi_info, abi_file_contents);
        }

        if events.len() == 1 {
            println!("One event found: \x1b[32m{}\x1b[0m", events[0]);
        } else {
            events = select_multiple_from_values("Select events", events.as_slice(), true);
        }

        // println!("Events: [{{{}}}]", events.into_iter().map(|x| x.to_string()).collect::<Vec<_>>().join("}, {"));

        let (proto_file, module_file, module_names) = get_proto_and_module_code_and_module_names(events, &spkg_name);

        // Add modules to substreams.yaml

        // and module file to mod.rs in modules folder

        // Write changes to file system
    }
}

fn get_proto(events: &Vec<Event>, spkg_pascal: &String, spkg_snake: &String) -> String {
    let mut proto = format!("syntax = \"proto3\";\n\
            package {};\
            \n\
            message {} {{\n", spkg_snake, spkg_pascal);

    for event in events {
        proto.push_str(event.)
    }

    String::new()
}

// fn rust_type(input: &ParamType) -> proc_macro2::TokenStream {
//     match *input {
//         ParamType::Address => quote! { Vec<u8> },
//         ParamType::Bytes => quote! { Vec<u8> },
//         ParamType::FixedBytes(size) => quote! { [u8; #size] },
//         ParamType::Int(_) => quote! { substreams::scalar::BigInt },
//         ParamType::Uint(_) => quote! { substreams::scalar::BigInt },
//         ParamType::Bool => quote! { bool },
//         ParamType::String => quote! { String },
//         ParamType::Array(ref kind) => {
//             let t = rust_type(&*kind);
//             quote! { Vec<#t> }
//         }
//         ParamType::FixedArray(ref kind, size) => {
//             let t = rust_type(&*kind);
//             quote! { [#t; #size] }
//         }
//         ParamType::Tuple(_) => {
//             unimplemented!(
//                 "Tuples are not supported. https://github.com/openethereum/ethabi/issues/175"
//             )
//         }
//     }
// }

fn assert_spkg_name_allowed(spkg_name: &String) {
    let chars_iter = spkg_name.chars();

    if let Some(first_char) = spkg_name.chars().next() {
        assert!(first_char.is_ascii_alphabetic(), "First character of supplied spkg name has to be alphabetical! (a-z || A-Z)\nSpkg value given: {}", spkg_name);
    }

    for char in chars_iter {
        assert!(char.is_ascii_alphanumeric() || char==' ' || char=='_', "Character: {} of supplied spkg name has to be alphabetical!\nSpkg value given: {}", char, spkg_name);
    }
}

fn to_pascal_case(string: String) -> String {
    let mut chars_iter = string.chars().into_iter();
    let mut pascal_string = String::new();
    if let Some(first_char) = chars_iter.next() {
        pascal_string.push(first_char.to_ascii_uppercase());
    }

    let mut prev_char_was_spacer = false;
    for char in chars_iter {
        if char=='_' || char==' ' {
            prev_char_was_spacer = true;
            continue;
        }

        if prev_char_was_spacer {
            prev_char_was_spacer = false;
            pascal_string.push(char.to_ascii_uppercase());
        } else {
            pascal_string.push(char);
        }
    }

    pascal_string
}

fn to_snake_case(string: String) -> String {
    let mut chars_iter = string.chars().into_iter();
    let mut snake_string = String::new();
    if let Some(first_char) = chars_iter.next() {
        snake_string.push(first_char.to_ascii_lowercase());
    }

    let mut prev_char_was_spacer = false;
    for char in chars_iter {
        if char=='_' || char==' ' {
            if !prev_char_was_spacer {
                prev_char_was_spacer = true;
                snake_string.push('_');
            }

            continue;
        }

        if prev_char_was_spacer {
            prev_char_was_spacer = false;
        } else if char.is_ascii_uppercase() {
            snake_string.push('_');
        }


        snake_string.push(char.to_ascii_lowercase());
    }

    snake_string
}

fn parse_events(contract: Contract) -> Vec<Event> {
    contract.events.into_iter().map(|(event_name, events)| {
        assert_eq!(events.len(), 1, "There should only be one event here!\nevent_name: {}\nevents: {:?}", event_name, events);
        let event = events.into_iter().next().unwrap();
        Event {
            event_name: event.name,
            event_params: event.inputs.into_iter().filter_map(|event_param| {
                if let ParamType::Tuple(_) = event_param.kind {
                    panic!("Tuples are not supported. https://github.com/openethereum/ethabi/issues/175");
                }

                if event_param.name.is_empty() {
                    None
                } else {
                    Some(EventParam {
                        param_name: event_param.name,
                        param_type: event_param.kind
                    })
                }
            }).collect()
        }
    }).collect()
}

struct DependencyManager {
    dependencies: HashMap<String, String>,
    panic_fn: fn(&String, &String, &String)
}

impl DependencyManager {
    fn proto_manager() -> Self {
        DependencyManager {
            dependencies: HashMap::new(),
            panic_fn: |dependency_type, existing_proto_content, new_proto_content| {
                panic!("Error adding proto dependency {} to dependency cash! Dependency content to add:\n{}\n\n\
                Although the following type has already been declared with the following content!:\n{}", dependency_type, new_proto_content, existing_proto_content);
            }
        }
    }

    fn impls_manager() -> Self {
        DependencyManager {
            dependencies: HashMap::new(),
            panic_fn: |dependency_type, existing_impl_content, new_impl_content| {
                panic!("Error adding impl dependency {} to dependency cash! Dependency content to add:\n{}\n\n\
                Although the following impl has already been declared with the following content!:\n{}", dependency_type, new_impl_content, existing_impl_content);
            }
        }
    }

    fn add_dependency(&mut self, dependency_type: String, content: String) {
        if self.dependencies.contains_key(&dependency_type) {
            if self.dependencies.get(&dependency_type).unwrap() != content {
                self.panic_fn(&dependency_type, &content, self.dependencies.get(&dependency_type).unwrap());
            }
        } else {
            self.dependencies.insert(dependency_type, content);
        }
    }

    fn get_dependencies(self) -> Vec<String> {
        let mut dependencies = self.dependencies.into_iter().map(|x| x.1).collect::<Vec<_>>();
        dependencies.sort();
        dependencies
    }
}

/// Returns a proto file for all event outputs, a code file for all the module code, and the names of the modules produces in form -> (proto_file, module_file, module_names)
fn get_proto_and_module_code_and_module_names(events: Vec<Event>, package_name: &String) -> (String, String, Vec<String>) {
    fn get_proto_field_type_and_mapping_code(param_type: &ParamType, proto_dependencies: &mut DependencyManager, impl_dependencies: &mut DependencyManager, package_name: &String, variable_path: String) -> (String, String) {
        match param_type {
            ParamType::Address => {
                ("string".to_string(), format!("hex::encode(&{})", variable_path))
            }
            ParamType::Bytes => {
                ("bytes".to_string(), variable_path)
            }
            ParamType::FixedBytes(_) => {
                ("bytes".to_string(), format!("{}.to_vec()", variable_path))
            }
            ParamType::Int(_) | ParamType::Uint(_) => {
                proto_dependencies.add_dependency("BigInt".to_string(), "message BigInt {\n    bytes bytes = 1;\n}\n".to_string());
                impl_dependencies.add_dependency("BigInt".to_string(), format!("impl From<BigInt> for {0}::BigInt {{\n\
                                                                                                        fn from(big_int: BigInt) -> Self {{\
                                                                                                            {0}::BigInt {{ bytes: big_int.to_bytes_le().1 }}\n\
                                                                                                        }}\n\
                                                                                                    }}\n", package_name));
                ("BigInt".to_string(), format!("{}.into()", variable_path))
            }
            ParamType::Bool => {
                ("bytes".to_string(), variable_path)
            }
            ParamType::String => {
                ("string".to_string(), variable_path)
            }
            ParamType::Array(inner_type) => {
                match inner_type.as_ref() {
                    ParamType::Address | ParamType::Int(_) | ParamType::Uint(_) | ParamType::FixedBytes(_) => {
                        let (proto_type, field_mapping_code) = get_proto_field_type_and_mapping_code(inner_type, proto_dependencies, impl_dependencies, package_name, "x".to_string());

                        (format!("repeated {}", proto_type), format!("{}.into_iter().map(|x| {}).collect::<Vec<_>>()", variable_path, field_mapping_code))
                    }
                    ParamType::Bytes => {
                        ("repeated bytes".to_string(), variable_path)
                    },
                    ParamType::Bool => {
                        ("repeated bool".to_string(), variable_path)
                    },
                    ParamType::String => {
                        ("repeated string".to_string(), variable_path)
                    }
                    ParamType::Array(param_type) | ParamType::FixedArray(param_type, _) => {
                        let inner_proto_type = get_proto_type(param_type, true);
                        let outer_proto_type = format!("Vec{}", to_pascal_case(proto_type.clone()));
                        let outer_proto_type_snake = to_snake_case(outer_proto_type);
                        let raw_type = get_type_display(param_type);

                        proto_dependencies.add_dependency(outer_proto_type.clone(), format!("message {} {{\n    repeated {} inner = 1;\n}}\n", outer_proto_type, inner_proto_type));
                        impl_dependencies.add_dependency(outer_proto_type.clone(), format!("impl From<{0}> for {1}::{2} {{\n    \
                                                                                                                    fn from({3}: {0}) -> Self {{        \
                                                                                                                        {1}::{2} {{ inner: {4} }}\n    \
                                                                                                                    }}\n\
                                                                                                                }}\n", raw_type, package_name, outer_proto_type, outer_proto_type_snake, get_proto_field_type_and_mapping_code(inner_type, proto_dependencies, impl_dependencies, package_name, outer_proto_type_snake).1));

                        (format!("repeated {}", outer_proto_type ), format!("{}.into_iter().map(|x| x.into()).collect::<Vec<_>>()", variable_path))
                    }
                    ParamType::Tuple(_) => unimplemented!(),
                }
            }
            ParamType::FixedArray(inner_type, _) => {
                match inner_type.as_ref() {
                    ParamType::Address | ParamType::Int(_) | ParamType::Uint(_) | ParamType::FixedBytes(_) => {
                        let (proto_type, field_mapping_code) = get_proto_field_type_and_mapping_code(inner_type, proto_dependencies, impl_dependencies, package_name, "x".to_string());

                        (format!("repeated {}", proto_type), format!("{}.into_iter().map(|x| {}).collect::<Vec<_>>()", variable_path, field_mapping_code))
                    }
                    ParamType::Bytes => {
                        ("repeated bytes".to_string(), format!("{}.to_vec()", variable_path))
                    },
                    ParamType::Bool => {
                        ("repeated bool".to_string(), format!("{}.to_vec()", variable_path))
                    },
                    ParamType::String => {
                        ("repeated string".to_string(), format!("{}.to_vec()", variable_path))
                    }
                    ParamType::Array(param_type) | ParamType::FixedArray(param_type, _) => {
                        let inner_proto_type = get_proto_type(param_type, true);
                        let outer_proto_type = format!("Vec{}", to_pascal_case(proto_type.clone()));
                        let outer_proto_type_snake = to_snake_case(outer_proto_type);
                        let raw_type = get_type_display(param_type);

                        proto_dependencies.add_dependency(outer_proto_type.clone(), format!("message {} {{\n    repeated {} inner = 1;\n}}\n", outer_proto_type, inner_proto_type));
                        impl_dependencies.add_dependency(outer_proto_type.clone(), format!("impl From<{0}> for {1}::{2} {{\n    \
                                                                                                                    fn from({3}: {0}) -> Self {{        \
                                                                                                                        {1}::{2} {{ inner: {4} }}\n    \
                                                                                                                    }}\n\
                                                                                                                }}\n", raw_type, package_name, outer_proto_type, outer_proto_type_snake, get_proto_field_type_and_mapping_code(inner_type, proto_dependencies, impl_dependencies, package_name, outer_proto_type_snake).1));

                        (format!("repeated {}", outer_proto_type ), format!("{}.into_iter().map(|x| x.into()).collect::<Vec<_>>()", variable_path))
                    }
                    ParamType::Tuple(_) => unimplemented!(),
                }
            }
            ParamType::Tuple(_) => unimplemented!()
        }
    }

    let mut proto_dependencies = DependencyManager::proto_manager();
    let mut impl_dependencies = DependencyManager::impls_manager();
    let mut events_protos = Vec::new();
    let mut event_protos = Vec::new();
    let mut modules = Vec::new();
    let mut module_names = Vec::new();
    for event in events.into_iter() {
        events_protos.push_str(&format!("message {0}s {{\n  \
                                                  repeated {0} items = 1;\n\
                                              }}\n", event.event_name));

        let mut event_proto = format!("message {0} {{\n  \
                                                string tx_hash = 1;\n  \
                                                uint32 log_index = 2;\n  \
                                                uint64 log_ordinal = 3;\n\n", event.event_name);

        let mut proto_initialization = format!("{}::{} {{\n                \
                                                          tx_hash: hex::encode(&log.receipt.transaction.hash),\n                \
                                                          log_index: log.index(),\n                \
                                                          log_ordinal: log.ordinal(),\n", package_name, event.event_name);

        let mut field_index = 4;
        for param in event.event_params.into_iter() {
            let (proto_field, mapping_code) = get_proto_field_type_and_mapping_code(&param.param_type, &mut proto_dependencies, &mut impl_dependencies, package_name, format!("event.{}", param.param_name));

            event_proto.push_str(&format!("  {} {} = {};\n", proto_field, param.param_name, field_index));
            field_index += 1;

            proto_initialization.push_str(&format!("                {}: {},\n", param.param_name, mapping_code));
        }

        event_proto.push_str("}\n");
        event_protos.push(event_proto);

        proto_initialization.push_str("            }");
        let event_name_snake = to_snake_case(event.event_name);
        let contract_address_ident = format!("{}_CONTRACT_ADDRESS", package_name.to_uppercase());
        let module_name = format!("map_{}s", event_name_snake);

        modules.push(format!("#[substreams::handlers::map]\n\
                                   fn {0}(block: eth::Block) -> Result<{1}::{2}s, substreams::errors::Error> {{\n    \
                                       let mut {3}s = {1}::{2}s {{ items: vec![] }};\n\
                                       \n    \
                                       for log in block.logs() {{\n        \
                                           if log.log.address != {4} {{\n            \
                                               continue;\n        \
                                           }}\n\
                                           \n        \
                                           if let Some(event) = abi::{1}::events::{2}::match_and_decode(log) {{\n            \
                                               {3}s.items.push({5})\n        \
                                           }}\n    \
                                       }}\n\
                                       \n    \
                                       Ok({}s)\n\
                                   }}\n", module_name, package_name, event.event_name, event_name_snake, contract_address_ident, proto_initialization));

    }

    // Check for BigInt and hex imports

    // Create proto file

    // Create module file

    // Return
    (String::new(), String::new(), Vec::new())
}

fn get_proto_type(param_type: &ParamType, is_first_call: bool) -> String {
    match param_type {
        ParamType::Address | ParamType::String => {
            if is_first_call {
                "string".to_string()
            } else {
                "String".to_string()
            }
        },
        ParamType::Bytes | ParamType::FixedBytes(_) => {
            if is_first_call {
                "bytes".to_string()
            } else {
                "Bytes".to_string()
            }
        },
        ParamType::Int(_) | ParamType::Uint(_) => "Bigint".to_string(),
        ParamType::Bool => {
            if is_first_call {
                "bool".to_string()
            } else {
                "Bool".to_string()
            }
        },
        ParamType::Array(inner_type) | ParamType::FixedArray(inner_type, _) => format!("Vec{}", get_proto_type(inner_type, false)),
        ParamType::Tuple(_) => unimplemented!()
    }
}

fn get_type_display(param_type: &ParamType) -> String {
    match param_type {
        ParamType::Address | ParamType::String => "String".to_string(),
        ParamType::Bytes => "Vec<u8>".to_string(),
        ParamType::Int(_) | ParamType::Uint(_) => "BigInt".to_string(),
        ParamType::Bool => "bool".to_string(),
        ParamType::Array(inner_type) => format!("Vec<{}>", get_type_display(inner_type)),
        ParamType::FixedBytes(size) => format!("[u8; {}]", size),
        ParamType::FixedArray(inner_type, size) => format!("{}; {}]", get_type_display(inner_type), size),
        ParamType::Tuple(_) => unimplemented!()
    }
}

#[derive(Clone)]
struct Event {
    event_name: String,
    event_params: Vec<EventParam>
}

#[derive(Clone)]
struct EventParam {
    param_name: String,
    param_type: ParamType
}

impl Display for Event {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Event: {}, Parameters: [{}]", self.event_name, self.event_params.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "))
    }
}

impl Display for EventParam {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}-{})", self.param_name, get_param_type_display(&self.param_type))
    }
}

fn get_param_type_display(param_type: &ParamType) -> String {
    match param_type {
        ParamType::Address => "Address".to_string(),
        ParamType::Bytes => "Bytes".to_string(),
        ParamType::Int(_) | ParamType::Uint(_) => "Bigint".to_string(),
        ParamType::Bool => "Bool".to_string(),
        ParamType::String => "String".to_string(),
        ParamType::Array(inner_type) => format!("Vec<{}>", get_param_type_display(inner_type)),
        ParamType::FixedBytes(_) => "Bytes".to_string(),
        ParamType::FixedArray(inner_type, _) => format!("Vec<{}>", get_param_type_display(inner_type)),
        ParamType::Tuple(_) => unimplemented!()
    }
}
