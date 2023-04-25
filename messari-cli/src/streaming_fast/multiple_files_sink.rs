use crate::streaming_fast::file::{File, Location};

pub(crate) trait MultipleFilesSink {
    fn process(&mut self, proto_data: &mut &[u8], block_number: i64) -> Result<Vec<File>, String>;
    fn flush_leftovers(&mut self, block_number: i64) -> Vec<File>;
    /// Resulting path used for calculating the start block_number based off previously processed data (if even multiple file types produced by the sink only one file type is needed for tracking)
    fn get_an_output_folder_location(&self) -> Location;
    fn set_starting_block_number(&mut self, starting_block_number: i64);
}