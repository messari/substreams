use clap::Parser;

use crate::abi::{add_abis, AbisArgs};
use crate::project_dir::ProjectDirArg;
use crate::protocols::ProtocolAndNetworkArgs;

#[derive(Parser)]
pub(crate) struct Abi {
    #[clap(flatten)]
    pub(crate) project_dir: ProjectDirArg,
    #[clap(flatten)]
    pub(crate) protocol_and_network_args: ProtocolAndNetworkArgs,
    #[clap(flatten)]
    pub(crate) abis_arg: AbisArgs,
}

impl Abi {
    pub(crate) fn execute(&mut self) {
        let project_dir = self.project_dir.get_project_dir(true);
        let protocol_and_network_info = self.protocol_and_network_args.get_info();

        add_abis(protocol_and_network_info, &self.abis_arg, &project_dir, true);
    }
}
