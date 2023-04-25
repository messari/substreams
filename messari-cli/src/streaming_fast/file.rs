use std::fmt::Display;
use std::fs;
use std::path::PathBuf;
use s3::Bucket;
use s3::creds::Credentials;
use tokio::fs::File as TokioFile;
use tokio::io::AsyncWriteExt;

use crate::streaming_fast::process_substream::EncodingType;

pub(crate) struct File {
    file_data: Vec<u8>,
    output_location: Location
}

impl File {
    pub(crate) fn new(file_data: Vec<u8>, output_location: Location) -> File {
        File {
            file_data,
            output_location,
        }
    }

    pub(crate) async fn save(self) {
        match self.output_location {
            Location::DataWarehouse(file_path) => {
                let bucket_name = "data-warehouse-load-427049689281-dev";
                let region = "us-west-2".parse().unwrap();
                let credentials = Credentials::default().unwrap();
                let bucket = Bucket::new(bucket_name, region, credentials).unwrap();
                let response_data = bucket.put_object(file_path.to_str().unwrap(), self.file_data.as_slice()).await.unwrap();
                assert_eq!(response_data.status_code(), 200, "Response was not successful!");
                println!("Data warehouse file uploaded!\nFilesize: {}B, Prefix: {}", get_file_size_string(self.file_data.len()), file_path.to_string_lossy());
            }
            Location::Local(file_path) => {
                let mut file = TokioFile::create(&file_path).await.unwrap();
                file.write_all(&self.file_data).await.unwrap();
                println!("Local file written!\nFilesize: {}B, Filepath: {}", get_file_size_string(self.file_data.len()), file_path.to_string_lossy());
            }
        }
    }
}

fn get_file_size_string(file_size: usize) -> String {
    if file_size < 1024 { // (<100B)
        format!("{}B", file_size)
    } else if file_size < 100*1024 { // (<100KB)
        format!("{:.2}KB", (file_size as f64)/1024)
    } else if file_size < 1024*1024 { // (<1MB)
        format!("{}KB", file_size)
    } else if 100*1024*1024 { // (<100MB)
        format!("{:.2}MB", (file_size as f64)/(1024*1024))
    } else if file_size < 1024*1024*1024 { // (<1GB)
        format!("{}MB", file_size)
    } else if 100*1024*1024*1024 { // (<100GB)
        format!("{:.2}GB", (file_size as f64)/(1024*1024*1024))
    } else { // (>100GB)
        // We are expecting to produce file around the block size of
        // 128MB so some of the above is already overkill here..
        format!("{:+e}B", file_size as f64)
    }
}

#[derive(Clone)]
pub(crate) enum Location {
    DataWarehouse(PathBuf),
    Local(PathBuf)
}

impl Location {
    pub(crate) fn new(location_type: LocationType, path: PathBuf) -> Location {
        match location_type {
            LocationType::DataWarehouse => Location::DataWarehouse(path),
            LocationType::Local => {
                fs::create_dir_all(&path).unwrap();
                Location::Local(path)
            }
        }
    }

    pub(crate) fn get_file_location<T: Display>(&self, first_block_number: i64, last_block_number: i64, encoding_type: &EncodingType) -> Location {
        let filename = match encoding_type {
            EncodingType::Parquet => format!("{}_{}.parquet", first_block_number, last_block_number)
        };

        match self {
            Location::DataWarehouse(base_path) => Location::DataWarehouse(base_path.join(filename)),
            Location::Local(base_path) => Location::Local(base_path.join(filename))
        }
    }
}

#[derive(Clone)]
pub(crate) enum LocationType {
    DataWarehouse,
    Local
}