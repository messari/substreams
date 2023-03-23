mod abi;
mod automapper;
mod command_line_client;
mod commands;
mod file_modification;
mod protocols;
mod template_files;
mod terminal_interface;
mod utils;
mod streaming_fast;

use std::fs;
use clap::Parser;
use prost::Message;
use prost_types::FileDescriptorProto;

use crate::command_line_client::CommandLineClient;


use crate::streaming_fast::streamingfast_dtos::Package;
use crate::streaming_fast::proto_structure_info::get_output_type_info;

#[tokio::main]
async fn main() {
    // let mut client: CommandLineClient = CommandLineClient::parse();
    // client.execute().await;

    let spkg = fs::read("test1-v0.1.0.spkg").unwrap();

    let package = Package::decode(spkg.as_ref()).unwrap();

    let mut proto_file = FileDescriptorProto::default();
    'a: for proto in package.proto_files.iter() {
        for proto_message in proto.message_type.iter() {
            println!("Message: {}", proto_message.name.as_ref().unwrap());
            if proto_message.name.as_ref().unwrap() == "PairCreatedEvent" {
                println!("ewfoinewpcfo");
                proto_file = proto.clone();
                break 'a;
            }
            // println!("Message: {}", proto_message.name.as_ref().unwrap());
            // if proto_message.name.as_ref().unwrap() == &message_type {
            //     return proto_message;
            // }
        }
    };

    println!("Proto: \n\n{:?}\n\n", proto_file);

    // for module in package.modules.as_ref().unwrap().modules.iter() {
    //     println!("Module name: {}", module.name);
    //     // if &module.name == module_name {
    //     //     println!("Module:\n\n{:?}\n\n", module);
    //     // }
    // }

    // println!("Protos:\n\n{:?}\n\n", package.proto_files);
}
