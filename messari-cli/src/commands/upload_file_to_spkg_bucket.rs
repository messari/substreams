use std::fs;
use std::path::PathBuf;
use std::process::Command;
use s3::Bucket;
use s3::creds::Credentials;

use clap::Parser;

#[derive(Parser)]
pub(crate) struct UploadFileToSpkgBucket {
    file_path: String
}

impl UploadFileToSpkgBucket {
    /// Builds a debian binary of the messari cli and then uploads it to aws
    pub(crate) async fn execute(&self) {
        let file_path = PathBuf::from(&self.file_path);
        if !file_path.exists() {
            panic!("The file path: {}, you gave here does not exist! Please specify a correct location for the file you want to upload!", self.file_path);
        }

        let file_data = fs::read(file_path).unwrap();
        let file_name = file_path.file_name().unwrap().to_string_lossy().to_string();

        let bucket_name = "spkg-bucket";
        let region = "us-west-2".parse().unwrap();
        let credentials = Credentials::default().unwrap();
        let bucket = Bucket::new(bucket_name, region, credentials).unwrap();
        let response_data = bucket.put_object(format!("/spkg-files/{}", file_name), file_data.as_slice()).await.unwrap();
        assert_eq!(response_data.status_code(), 200, "Response was not successful!");
        println!("File: {}, has now been uploaded!\nFilesize: {}", file_name, get_file_size_string(file_data.len()));
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