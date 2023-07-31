use clap::{Parser, ValueEnum};
use std::fs;
use std::path::PathBuf;

use crate::streaming_fast::process_substream::{EncodingType, process_substream};
use crate::streaming_fast::file::LocationType as Location;
use crate::streaming_fast::streaming_config::ConfigArg;

#[derive(Parser)]
pub(crate) struct Process {
    spkg_path: String,
    #[clap(flatten)]
    config: ConfigArg,
    #[arg(short, long, value_name = "Location Type", help="Defaults to saving to local filepath.")]
    location_type: Option<LocationType>,
    #[arg(short, long, value_name = "Data location path", help="If not specified it will default to substreams on aws and ./data/ on local.")]
    data_location_path: Option<String>,
    #[arg(short, long, value_name = "Bucket", help="Mandatory if location type is DWH")]
    bucket: Option<String>,
    #[arg(short, long, value_name = "Start Block")]
    start_block: Option<i64>,
    #[arg(short, long, value_name = "Stop Block")]
    stop_block: Option<u64>,
}

#[derive(ValueEnum, Clone)]
pub(crate) enum LocationType {
    Local,
    Dwh
}

impl Process {
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
            Some(LocationType::Dwh) => {
                if self.bucket.is_none() {
                    panic!("Bucket is mandatory if location type is DWH");
                }
                Location::DataWarehouse
            },
        };

        let data_location_path = self.data_location_path.clone().map(|path| PathBuf::from(path));

        process_substream(spkg_data, config, EncodingType::Parquet, location_type, data_location_path, self.bucket.clone(), self.start_block, self.stop_block).await;

        println!("Processing complete!!!");
    }
}