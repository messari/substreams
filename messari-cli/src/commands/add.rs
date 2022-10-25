use clap::Parser;
use crate::abi::{AbisArg, add_abis};
use crate::protocols::ProtocolAndNetworkArgs;

#[derive(Parser)]
pub(crate) struct Add {
    #[clap(flatten)]
    pub(crate) protocol_and_network_args: ProtocolAndNetworkArgs,
    #[clap(flatten)]
    pub(crate) abis_arg: AbisArg
}

impl Add {
    pub(crate) fn execute(&mut self) {
        let protocol_and_network_info = self.protocol_and_network_args.get_info();
        add_abis(protocol_and_network_info, &self.abis_arg, &project_dir);
    }
}