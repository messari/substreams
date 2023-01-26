mod abi;
mod automapper;
mod command_line_client;
mod commands;
mod file_modification;
mod protocols;
mod template_files;
mod terminal_interface;
mod utils;
mod project_dir;

use clap::Parser;

use crate::command_line_client::CommandLineClient;

fn main() {
    let mut client: CommandLineClient = CommandLineClient::parse();
    client.execute();
}
