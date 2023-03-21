use clap::{Parser, ValueEnum};
use std::fs;
use std::path::PathBuf;

use crate::streaming_fast::process_substream::{EncodingType, process_substream};
use crate::streaming_fast::file::LocationType as Location;

#[derive(Parser)]
pub(crate) struct Process {
    spkg_path: String,
    module_name: String,
    #[arg(short, long, value_name = "Location Type", help="Defaults to saving to local filepath.")]
    location_type: Option<LocationType>,
    #[arg(short, long, value_name = "Data location path", help="TODO...")]
    data_location_path: Option<String>,
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

        let spkg_data = fs::read(spkg_path).unwrap();
        let location_type = match self.location_type {
            None => Location::Local,
            Some(LocationType::Local) => Location::Local,
            Some(LocationType::Dwh) => Location::DataWarehouse,
        };

        let data_location_path = self.data_location_path.clone().map(|path| PathBuf::from(path));

        process_substream(spkg_data, self.module_name.clone(), EncodingType::Parquet, location_type, data_location_path, self.start_block, self.stop_block).await;

        println!("Processing complete!!!");
    }
}