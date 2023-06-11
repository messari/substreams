use std::fs;
use std::path::PathBuf;
use s3::Bucket;
use s3::creds::Credentials;
use clap::Parser;

use crate::streaming_fast::streaming_fast_utils::get_file_size_string;

#[derive(Parser)]
pub(crate) struct UploadCliToAws {
    cli_file_path: String
}

impl UploadCliToAws {
    /// Builds a debian binary of the messari cli and then uploads it to aws
    pub(crate) async fn execute(&self) {
        let cli_file_path = PathBuf::from(&self.cli_file_path);
        if !cli_file_path.exists() {
            panic!("The config file path: {}, you gave here does not exist! Please specify a correct location for the config file you want to upload!", self.cli_file_path);
        }

        let file_data = fs::read(&cli_file_path).unwrap();
        let file_name = cli_file_path.file_name().unwrap().to_string_lossy().to_string();
        assert_eq!(file_name.as_str(), "messari", "CLI needs to be called messari!");

        let bucket_name = "spkg-bucket";
        let region = "us-west-2".parse().unwrap();
        let credentials = Credentials::default().unwrap();
        let bucket = Bucket::new(bucket_name, region, credentials).unwrap();
        let response_data = bucket.put_object("/messari-cli/messari", file_data.as_slice()).await.unwrap();
        assert_eq!(response_data.status_code(), 200, "Response was not successful!");
        println!("messari CLI has now been uploaded!\nFilesize: {}", get_file_size_string(file_data.len()));
    }
}