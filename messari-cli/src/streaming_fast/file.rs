use std::fs;
use std::path::PathBuf;
use tokio::fs::File as TokioFile;
use tokio::io::AsyncWriteExt;

use crate::streaming_fast::process_substream::EncodingType;
use crate::streaming_fast::streaming_fast_utils::get_file_size_string;

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
            Location::DataWarehouse(file_path, bucket_name) => {
                let region = "us-west-2";
                let file_data_len = self.file_data.len();

                aws_sdk_s3::Client::new(&aws_config::from_env().region(region).load().await)
                    .put_object()
                    .bucket(bucket_name)
                    .key(file_path.to_str().unwrap())
                    .body(self.file_data.into())
                    .send()
                    .await
                    .unwrap();

                println!("S3 file uploaded!\nFilesize: {}B, Prefix: {}", get_file_size_string(file_data_len), file_path.to_string_lossy());
            }
            Location::Local(file_path) => {
                let mut file = TokioFile::create(&file_path).await.unwrap();
                file.write_all(&self.file_data).await.unwrap();
                println!("Local file written!\nFilesize: {}B, Filepath: {}", get_file_size_string(self.file_data.len()), file_path.to_string_lossy());
            }
        }
    }
}

#[derive(Clone)]
pub(crate) enum Location {
    DataWarehouse(PathBuf, String),
    Local(PathBuf)
}

impl Location {
    pub(crate) fn new(location_type: LocationType, path: PathBuf, bucket_name: Option<String>) -> Location {
        match location_type {
            LocationType::DataWarehouse => Location::DataWarehouse(path, bucket_name.unwrap()),
            LocationType::Local => {
                fs::create_dir_all(&path).unwrap();
                Location::Local(path)
            }
        }
    }

    pub(crate) fn get_file_location(&self, first_block_number: i64, last_block_number: i64, encoding_type: &EncodingType) -> Location {
        let filename = match encoding_type {
            EncodingType::Parquet => format!("{}_{}.parquet", first_block_number, last_block_number)
        };

        match self {
            Location::DataWarehouse(base_path, bucket_name) => Location::DataWarehouse(base_path.join(filename), bucket_name.to_string()),
            Location::Local(base_path) => Location::Local(base_path.join(filename))
        }
    }
}

#[derive(Clone)]
pub(crate) enum LocationType {
    DataWarehouse,
    Local
}