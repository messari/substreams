use crate::streaming_fast::file::File;

pub(crate) trait MultipleFilesSink {
    fn process(&mut self, proto_data: &mut &[u8], block_number: i64) -> Result<Vec<File>, String>;
    fn flush_leftovers(&mut self, block_number: i64) -> Vec<File>;
}