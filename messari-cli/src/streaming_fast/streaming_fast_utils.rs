use std::fmt::Debug;
use std::fs;
#[cfg(test)]
use derives::TestData;
use s3::Bucket;
use s3::creds::Credentials;
use crate::streaming_fast::file::Location;

use crate::streaming_fast::file_sinks::parquet::ParquetFileSink;
use crate::streaming_fast::streamingfast_dtos::Package;

/// Considers all output folder paths and takes the earliest start block from all output folders as the global start block
pub(crate) async fn get_start_block_num(output_folder_paths: Vec<Location>, fallback_starting_block: i64) -> i64 {
    get_start_block_numbers(output_folder_paths, fallback_starting_block).await.into_iter().min().unwrap()
}

pub(crate) fn get_initial_block_for_module(package: &Package, proto_type_name: &str) -> i64 {
    for module in package.modules.as_ref().unwrap().modules.iter() {
        if module.output.is_some() && module.output.as_ref().unwrap().r#type.as_str() == proto_type_name {
            return module.initial_block as i64;
        }
    }

    panic!("Unable to match the module output: {} to a given module!", proto_type_name);
}

/// Returns a list of block number where each block number is the starting block for the
/// corresponding output folder path in the input list for the same element number
pub(crate) async fn get_start_block_numbers(output_folder_paths: Vec<Location>, fallback_starting_block: i64) -> Vec<i64> {
    let mut starting_block_numbers = Vec::new();

    for output_folder_path in output_folder_paths.into_iter() {
        let processed_block_files = match output_folder_path {
            Location::DataWarehouse(path) => {
                let bucket_name = "data-warehouse-load-427049689281-dev";
                let region = "us-west-2".parse().unwrap();
                let credentials = Credentials::default().unwrap();
                let bucket = Bucket::new(bucket_name, region, credentials).unwrap();
                let list_response = bucket.list(path.to_string_lossy().to_string(), None).await.unwrap();
                list_response.into_iter().map(|x| x.name).collect::<Vec<_>>()
            }
            Location::Local(path) => {
                fs::read_dir(path).unwrap().into_iter().map(|path| path.unwrap().path().display().to_string()).collect::<Vec<_>>()
            }
        };

        if processed_block_files.len() > 0 {
            // For now we will just assume all files will be in form -> startBlock_stopBlock.fileExtension
            let mut last_block_num_iterator = processed_block_files.into_iter().map(|file| {
                file.split('.').next().unwrap().split('_').last().unwrap().parse::<i64>().unwrap()
            });
            let mut latest_block_num = last_block_num_iterator.next().unwrap();
            for block_num in last_block_num_iterator {
                if block_num > latest_block_num {
                    latest_block_num = block_num;
                }
            }
            starting_block_numbers.push(latest_block_num);
        } else {
            starting_block_numbers.push(fallback_starting_block);
        }
    }

    starting_block_numbers
}

#[cfg(test)]
pub(crate) fn assert_data_sinks_to_parquet_correctly<T: TestData + Debug>() {
    use crate::streaming_fast::file_sinks::file_sink::FileSink;
    use parquet::file::reader::{FileReader, SerializedFileReader};
    use rand::rngs::StdRng;
    use rand::SeedableRng;
    use rand::Rng;

    const NUM_SAMPLES: usize = 50;

    let mut rng = StdRng::seed_from_u64(42);
    let test_data = T::get_samples(NUM_SAMPLES, &mut rng);
    let test_block_numbers = (0..NUM_SAMPLES).into_iter().map(|_| rng.gen()).collect::<Vec<i64>>();

    let mut sink = ParquetFileSink::new(T::get_proto_structure_info());
    for (test_datum, test_block_number) in test_data.iter().zip(test_block_numbers.iter()) {
        let bytes: Vec<u8> = test_datum.to_proto_bytes();

        sink.process(&mut bytes.as_slice(), *test_block_number).unwrap();
    }

    let parquet_file_data = sink.make_file();
    let reader = SerializedFileReader::new(bytes::Bytes::from(parquet_file_data)).unwrap();

    for ((parquet_row, test_datum), test_block_number) in reader.get_row_iter(None).unwrap().zip(test_data).zip(test_block_numbers.into_iter()) {
        let (parsed_data, block_number_result) = T::get_from_parquet_row(parquet_row.get_column_iter());
        assert!(block_number_result.is_some(), "Unable to parse block number from parquet row!\nParsed data: {:?}", parsed_data);
        assert_eq!(parsed_data, test_datum);
        assert_eq!(block_number_result.unwrap() as i64, test_block_number);
    }
}

pub(crate) fn get_file_size_string(file_size: usize) -> String {
    if file_size < 1024 { // (<100B)
        format!("{}B", file_size)
    } else if file_size < 100*1024 { // (<100KB)
        format!("{:.2}KB", (file_size as f64)/1024_f64)
    } else if file_size < 1024*1024 { // (<1MB)
        format!("{}KB", file_size)
    } else if file_size < 100*1024*1024 { // (<100MB)
        format!("{:.2}MB", (file_size as f64)/(1024_f64*1024_f64))
    } else if file_size < 1024*1024*1024 { // (<1GB)
        format!("{}MB", file_size)
    } else if file_size < 100*1024*1024*1024 { // (<100GB)
        format!("{:.2}GB", (file_size as f64)/(1024_f64*1024_f64*1024_f64))
    } else { // (>100GB)
        // We are expecting to produce file around the block size of
        // 128MB so some of the above is already overkill here..
        format!("{:+e}B", file_size as f64)
    }
}

pub(crate) trait FromSignedVarint: Sized
{
    fn from_signed_varint(data: &mut &[u8]) -> Option<Self>;
}

impl<T: Default + TryFrom<i64>> FromSignedVarint for T
    where
        T::Error: Debug,
{
    fn from_signed_varint(data: &mut &[u8]) -> Option<Self>
    {
        u64::from_unsigned_varint(data).map(|u| {
            let signed: i64 = unsafe { std::mem::transmute(u) };
            signed.try_into().unwrap()
        })
    }
}

pub(crate) trait FromUnsignedVarint: Sized
{
    fn from_unsigned_varint(data: &mut &[u8]) -> Option<Self>;
}

impl<T: Default + TryFrom<u64>> FromUnsignedVarint for T
    where
        T::Error: Debug,
{
    fn from_unsigned_varint(data: &mut &[u8]) -> Option<Self>
    {
        let mut result = 0u64;
        let mut idx = 0;
        loop {
            if idx >= data.len() {
                return None;
            }

            let b = data[idx];
            let value = (b & 0x7f) as u64;
            result += value << (idx * 7);

            idx += 1;
            if b & 0x80 == 0 {
                break;
            }
        }

        let result = T::try_from(result).expect("Out of range");
        *data = &data[idx..];
        Some(result)
    }
}