use crate::file::File;

pub(crate) trait Sink {
    /// If during a process of a new block output a DWH file turns ready
    /// to upload it will be returned from the function signature
    fn process(&mut self, proto_data: Vec<u8>, block_number: i64) -> Result<Option<File>, String>;

    /// Essentially flushes whatever data the encoder currently has into a File
    fn make_file(&mut self) -> File;
}