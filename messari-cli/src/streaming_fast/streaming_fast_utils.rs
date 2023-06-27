use std::fmt::Debug;
use futures::StreamExt;
use parquet::file::reader::{FileReader, SerializedFileReader};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use derives::TestData;

use crate::streaming_fast::file_sinks::file_sink::FileSink;
use crate::streaming_fast::file_sinks::parquet::ParquetFileSink;

const DUMMY_BLOCK_NUMBER: i64 = 1;

#[cfg(test)]
pub(crate) fn assert_data_sinks_to_parquet_correctly<T: TestData + Debug>() {
    const NUM_SAMPLES: usize = 50;

    let mut rng = StdRng::seed_from_u64(42);
    let mut test_data = T::get_samples(NUM_SAMPLES, &mut rng);
    let test_block_numbers = (0..NUM_SAMPLES).into_iter().map(|_| rng.gen()).collect::<Vec<i64>>();

    let mut sink = ParquetFileSink::new(T::get_proto_structure_info());
    for (test_datum, test_block_number) in test_data.iter().zip(test_block_numbers.iter()) {
        let bytes: Vec<u8> = test_datum.to_proto_bytes();

        sink.process(&mut bytes.as_slice(), *test_block_number).unwrap();
    }

    let parquet_file_data = sink.make_file();
    let reader = SerializedFileReader::new(bytes::Bytes::from(parquet_file_data)).unwrap();

    for ((parquet_row, test_datum), test_block_number) in reader.get_row_iter(None).unwrap().zip(test_data).zip(test_block_numbers.into_iter()) {
        let (parsed_data, block_number_result) = T::get_from_parquet_row(parquet_row);
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