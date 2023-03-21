use crate::commands::add::Add;
use clap::Parser;

use crate::commands::init::Init;
use crate::commands::process::Process;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) enum CommandLineClient {
    Init(Init),
    Add(Add),
    Process(Process)
}

impl CommandLineClient {
    pub(crate) async fn execute(&mut self) {
        match self {
            CommandLineClient::Init(cmd) => cmd.execute(),
            CommandLineClient::Add(cmd) => cmd.execute(),
            CommandLineClient::Process(cmd) => cmd.execute().await,
        }
    }
}
