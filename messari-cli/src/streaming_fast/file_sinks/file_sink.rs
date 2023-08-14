use derives::proto_structure_info::MessageInfo;

pub(crate) trait FileSink: Send + Sync {
    /// Initialises itself from a message type field
    fn new(output_type_info: MessageInfo) -> Self where Self: Sized;

    /// If during a process of a new block output a the building file reaches the right size then it will return the file content
    fn process(&mut self, proto_data: &mut &[u8], block_number: i64) -> Result<Option<Vec<u8>>, String>;

    /// Essentially flushes whatever data the encoder currently has into a file
    fn make_file(&mut self) -> Vec<u8>;
}