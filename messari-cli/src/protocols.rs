use crate::terminal_interface::{select_from_enum, select_from_values};
use clap::Parser;
use clap::ValueEnum;
use std::fmt::{Display, Formatter};
use strum_macros::{AsRefStr, EnumIter, EnumVariantNames};

#[derive(ValueEnum, Clone, EnumIter, EnumVariantNames, AsRefStr)]
pub(crate) enum ProtocolType {
    Arweave,
    Ethereum,
    Near,
    Cosmos,
}

#[derive(Parser)]
pub(crate) struct ProtocolAndNetworkArgs {
    #[arg(short, long, value_name = "Protocol")]
    pub(crate) protocol_type: Option<ProtocolType>,
    #[arg(short = 'k', long, value_name = "Network")]
    pub(crate) network: Option<String>,
}

impl ProtocolAndNetworkArgs {
    pub(crate) fn get_info(&self) -> ProtocolAndNetworkInfo {
        let protocol_type = if self.protocol_type.is_none() {
            let protocol: ProtocolType = select_from_enum("Protocol", Some(0));
            protocol
        } else {
            self.protocol_type.as_ref().unwrap().clone()
        };

        let protocol = protocol_type.get_protocol();

        let network = if let Some(network) = &self.network {
            if !protocol.available_networks.contains(&network.as_str()) {
                panic!("Network supplied: {} is an invalid network relative to the protocol supplied!: {}", protocol, network);
            }
            network.clone()
        } else {
            let network = select_from_values("Network", &protocol.available_networks, None);
            network.to_string()
        };

        ProtocolAndNetworkInfo { protocol, network }
    }
}

pub(crate) struct ProtocolAndNetworkInfo {
    pub(crate) protocol: Protocol,
    pub(crate) network: String,
}

impl ProtocolType {
    pub(crate) fn get_protocol(&self) -> Protocol {
        match self {
            ProtocolType::Arweave => Protocol {
                protocol_type: ProtocolType::Arweave,
                available_networks: vec!["arweave-mainnet"],
                supported_abi_addition_methods: SupportedAbiAdditionMethods::CopyFromLocalFilePath,
            },
            ProtocolType::Ethereum => Protocol {
                protocol_type: ProtocolType::Ethereum,
                available_networks: vec![
                    "mainnet",
                    "rinkeby",
                    "goerli",
                    "poa-core",
                    "poa-sokol",
                    "gnosis",
                    "matic",
                    "mumbai",
                    "fantom",
                    "fantom-testnet",
                    "bsc",
                    "chapel",
                    "clover",
                    "avalanche",
                    "fuji",
                    "celo",
                    "celo-alfajores",
                    "fuse",
                    "moonbeam",
                    "moonriver",
                    "mbase",
                    "arbitrum-one",
                    "arbitrum-rinkeby",
                    "optimism",
                    "optimism-kovan",
                    "aurora",
                    "aurora-testnet",
                ],
                supported_abi_addition_methods:
                    SupportedAbiAdditionMethods::ByEitherLocalOrDownload,
            },
            ProtocolType::Near => Protocol {
                protocol_type: ProtocolType::Near,
                available_networks: vec!["near-mainnet", "near-testnet"],
                supported_abi_addition_methods: SupportedAbiAdditionMethods::CopyFromLocalFilePath,
            },
            ProtocolType::Cosmos => {
                Protocol {
                    protocol_type: ProtocolType::Cosmos,
                    available_networks: vec![
                        "cosmoshub-4",
                        "theta-testnet-001", // CosmosHub testnet
                        "osmosis-1",
                        "osmo-test-4", // Osmosis testnet
                        "juno-1",
                        "uni-3", // Juno testnet
                    ],
                    supported_abi_addition_methods:
                        SupportedAbiAdditionMethods::CopyFromLocalFilePath,
                }
            }
        }
    }
}

pub(crate) struct Protocol {
    pub(crate) protocol_type: ProtocolType,
    pub(crate) available_networks: Vec<&'static str>,
    pub(crate) supported_abi_addition_methods: SupportedAbiAdditionMethods,
}

#[allow(dead_code)]
pub(crate) enum SupportedAbiAdditionMethods {
    CopyFromLocalFilePath,
    DownloadFromContractAddress,
    ByEitherLocalOrDownload, // when both types of addition can be supported
}

impl Display for Protocol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.protocol_type.as_ref())
    }
}
