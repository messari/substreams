use clap::Parser;
use crate::commands::add::Add;

use crate::commands::init::Init;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) enum CommandLineClient {
    Init(Init),
    Add(Add)
}

impl CommandLineClient {
    pub(crate) fn execute(&mut self) {
        match self {
            CommandLineClient::Init(cmd) => cmd.execute(),
            CommandLineClient::Add(cmd) => cmd.execute()
        }
    }
}