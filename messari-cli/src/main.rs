mod command_line_client;
mod commands;
mod protocols;
mod cmd_helper;
mod utils;
mod abi;
mod automapper;
mod file_contents_modifier;
mod cargo;

use clap::Parser;

use crate::command_line_client::CommandLineClient;

fn main() {
    let mut client: CommandLineClient = CommandLineClient::parse();
    client.execute();
}
