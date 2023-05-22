use crate::commands::add::Add;
use clap::Parser;
use crate::commands::block_range_info::BlockRangeInfo;
use crate::commands::build_and_upload_new_or_modified_substreams::BuildAndUploadNewOrModifiedSubstreams;

use crate::commands::init::Init;
use crate::commands::process::Process;
use crate::commands::update_versions::UpdateVersions;
use crate::commands::upload_cli_to_aws::UploadCliToAws;
use crate::commands::upload_config_and_spkg_to_bucket::UploadConfigAndSpkgToAws;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) enum CommandLineClient {
    Init(Init),
    Add(Add),
    Process(Process),
    BlockRangeInfo(BlockRangeInfo),
    UploadConfigAndSpkgToAws(UploadConfigAndSpkgToAws),
    UploadCliToAws(UploadCliToAws),
    UpdateVersions(UpdateVersions),
    BuildAndUploadNewOrModifiedSubstreams(BuildAndUploadNewOrModifiedSubstreams)
}

impl CommandLineClient {
    pub(crate) async fn execute(&mut self) {
        match self {
            CommandLineClient::Init(cmd) => cmd.execute(),
            CommandLineClient::Add(cmd) => cmd.execute(),
            CommandLineClient::Process(cmd) => cmd.execute().await,
            CommandLineClient::BlockRangeInfo(cmd) => cmd.execute().await,
            CommandLineClient::UploadConfigAndSpkgToAws(cmd) => cmd.execute().await,
            CommandLineClient::UploadCliToAws(cmd) => cmd.execute().await,
            CommandLineClient::UpdateVersions(cmd) => cmd.execute(),
            CommandLineClient::BuildAndUploadNewOrModifiedSubstreams(cmd) => cmd.execute().await
        }
    }
}
