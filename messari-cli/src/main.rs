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

use clap::Parser;

use crate::command_line_client::CommandLineClient;

#[tokio::main]
async fn main() {
    let mut client: CommandLineClient = CommandLineClient::parse();
    client.execute().await;
}




