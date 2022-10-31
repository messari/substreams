mod abi;
mod automapper;
mod terminal_interface;
mod command_line_client;
mod commands;
mod protocols;
mod utils;
mod file_modification;

use clap::Parser;

use crate::command_line_client::CommandLineClient;

fn main() {
    let mut client: CommandLineClient = CommandLineClient::parse();
    client.execute();
}
    