use clap::Parser;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tonic::transport::Uri;

#[derive(Parser)]
pub(crate) struct ConfigArg {
    #[arg(short, long)]
    config: String, // Mandatory to specify a config for each spkg
}

impl ConfigArg {
    pub(crate) fn parse(&self) -> StreamingConfig {
        serde_json::from_str(self.config.as_str()).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct StreamingConfig {
    pub(crate) name: Option<String>,
    pub(crate) output_module: String,
    pub(crate) substream_name_override: Option<String>,
    pub(crate) chain_override: Option<Chain>,
    pub(crate) param_overrides: Vec<ParamOverride>,
    pub(crate) start_block_overrides: Vec<StartBlockOverride>,
}

impl StreamingConfig {
    pub(crate) fn get_start_block_override(&self) -> Option<i64> {
        let mut start_block_override = None;
        for start_override in self.start_block_overrides.iter() {
            if start_override.module == self.output_module {
                start_block_override = Some(start_override.block_number as i64);
                break;
            }
        }
        start_block_override
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct StartBlockOverride {
    pub(crate) module: String,
    pub(crate) block_number: u64,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ParamOverride {
    pub(crate) module: String,
    pub(crate) value: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) enum Chain {
    #[serde(rename = "mainnet")]
    // We will revert to ethereum-mainnet once other chains are added, but for now it's nicer just to have as mainnet
    EthereumMainnet,
    Polygon,
}

impl Default for Chain {
    fn default() -> Self {
        Chain::EthereumMainnet
    }
}

impl Chain {
    pub(crate) fn get_proto_block_type(&self) -> String {
        match self {
            Chain::EthereumMainnet => "sf.ethereum.type.v2.Block".to_string(),
            Chain::Polygon => "sf.ethereum.type.v2.Block".to_string(),
        }
    }

    pub(crate) fn get_endpoint(&self) -> Uri {
        match self {
            Chain::EthereumMainnet => Uri::from_static("https://mainnet.eth.streamingfast.io:443"),
            Chain::Polygon => Uri::from_static("https://polygon.streamingfast.io:443"),
        }
    }

    pub(crate) fn default_for_block_type(block_type_str: &str) -> Self {
        match block_type_str {
            "sf.ethereum.type.v2.Block" => Chain::EthereumMainnet,
            _ => panic!(
                "Unable to identify a default chain for input block type: {}!",
                block_type_str
            ),
        }
    }

    pub(crate) fn add_chain_folders_to_path(&self, path: PathBuf) -> PathBuf {
        match self {
            Chain::EthereumMainnet => path.join("ethereum").join("mainnet"),
            Chain::Polygon => path.join("ethereum").join("polygon"),
        }
    }
}

pub(crate) trait ToJsonL {
    fn to_jsonl(self) -> String;
}

impl ToJsonL for Vec<StreamingConfig> {
    fn to_jsonl(self) -> String {
        let json_lines = self
            .iter()
            .map(|config_profile| serde_json::to_string(config_profile).unwrap())
            .collect::<Vec<_>>();
        json_lines.join("\n")
    }
}
