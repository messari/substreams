use crate::streaming_fast::file::{File, Location};
use async_trait::async_trait;

#[async_trait]
pub(crate) trait MultipleFilesSink {
    fn process(&mut self, proto_data: &mut &[u8], block_number: i64) -> Result<Vec<File>, String>;
    fn flush_leftovers(&mut self, block_number: i64) -> Vec<File>;
    fn get_output_folder_locations(&self) -> Vec<Location>;
    fn get_bucket_name(&self) -> Option<String>;
    fn notify_new_block(&mut self, block_number: i64);
    async fn set_starting_block_number(&mut self, starting_block_number: i64);
}