use clap::{Parser, ValueEnum};
use std::fs;
use std::path::PathBuf;

use crate::streaming_fast::process_substream::get_block_range_info;
use crate::streaming_fast::file::LocationType as Location;
use crate::streaming_fast::streaming_config::ConfigArg;

#[derive(Parser)]
pub(crate) struct BlockRangeInfo {
    spkg_path: String,
    #[clap(flatten)]
    config: ConfigArg,
    #[arg(short, long, value_name = "Location Type", help="Defaults to checking local filepath.")]
    location_type: Option<LocationType>,
    #[arg(short, long, value_name = "Data location path", help="If not specified it will default to check substreams on aws and ./data/ on local.")]
    data_location_path: Option<String>,
}

#[derive(ValueEnum, Clone)]
pub(crate) enum LocationType {
    Local,
    Dwh
}

impl BlockRangeInfo {
    pub(crate) async fn execute(&self) {
        let spkg_path = PathBuf::from(&self.spkg_path);
        if !spkg_path.exists() {
            panic!("The spkg path: {}, you gave here does not exist! Please specify a correct location for the spkg path!", self.spkg_path);
        }

        let config = self.config.parse();

        let spkg_data = fs::read(spkg_path).unwrap();
        let location_type = match self.location_type {
            None => Location::Local,
            Some(LocationType::Local) => Location::Local,
            Some(LocationType::Dwh) => Location::DataWarehouse,
        };

        let data_location_path = self.data_location_path.clone().map(|path| PathBuf::from(path));

        let (start_block, stop_block) = get_block_range_info(spkg_data, config.output_module.as_str(), location_type, data_location_path, config.get_start_block_override(), config.chain_override).await;

        println!("{{start_block: {}, stop_block: {}}}", start_block, stop_block);
    }
}

