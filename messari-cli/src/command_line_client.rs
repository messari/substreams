use crate::commands::add::Add;
use clap::Parser;
use crate::commands::block_range_info::BlockRangeInfo;

use crate::commands::init::Init;
use crate::commands::process::Process;
use crate::commands::upload_file_to_spkg_bucket::UploadFileToSpkgBucket;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) enum CommandLineClient {
    Init(Init),
    Add(Add),
    Process(Process),
    BlockRangeInfo(BlockRangeInfo),
    UploadFileToSpkgBucket(UploadFileToSpkgBucket)
}

impl CommandLineClient {
    pub(crate) async fn execute(&mut self) {
        match self {
            CommandLineClient::Init(cmd) => cmd.execute(),
            CommandLineClient::Add(cmd) => cmd.execute(),
            CommandLineClient::Process(cmd) => cmd.execute().await,
            CommandLineClient::BlockRangeInfo(cmd) => cmd.execute().await,
            CommandLineClient::UploadFileToSpkgBucket(cmd) => cmd.execute().await
        }
    }
}
