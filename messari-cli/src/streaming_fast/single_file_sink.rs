use std::path::PathBuf;
use derives::proto_structure_info::MessageInfo;

use crate::streaming_fast::file::{File, Location, LocationType};
use crate::streaming_fast::file_sinks::file_sink::FileSink;
use crate::streaming_fast::file_sinks::parquet::ParquetFileSink;
use crate::streaming_fast::multiple_files_sink::MultipleFilesSink;
use crate::streaming_fast::process_substream::EncodingType;

pub(crate) struct SingleFileSink {
    file_sink: Box<dyn FileSink>,
    sink_output_location: Location,
    starting_block_number: i64,
    encoding_type: EncodingType,
}

impl SingleFileSink {
    pub(crate) fn new(output_type_info: MessageInfo, encoding_type: EncodingType, location_type: LocationType, mut sink_output_path: PathBuf) -> Self {
        sink_output_path = sink_output_path.join(&output_type_info.type_name);

        let file_sink = match encoding_type {
            EncodingType::Parquet => {
                sink_output_path = sink_output_path.join("parquet");
                Box::new(ParquetFileSink::new(output_type_info))
            }
        };

        let sink_output_location = Location::new(location_type, sink_output_path);

        SingleFileSink {
            file_sink,
            sink_output_location,
            starting_block_number: 0, // 0 set initially as a dummy value - will be overwritten later on
            encoding_type,
        }
    }
}

impl MultipleFilesSink for SingleFileSink {
    fn process(&mut self, proto_data: &mut &[u8], block_number: i64) -> Result<Vec<File>, String> {
        if let Some(file_data) = self.file_sink.process(proto_data, block_number)? {
            let output_files = vec![File::new(file_data, self.sink_output_location.get_file_location(self.starting_block_number, block_number, &self.encoding_type))];
            self.starting_block_number = block_number + 1;
            Ok(output_files)
        } else {
            Ok(Vec::new())
        }
    }

    fn flush_leftovers(&mut self, block_number: i64) -> Vec<File> {
        let file_data = self.file_sink.make_file();

        let mut output = Vec::new();
        if !file_data.is_empty() {
            output.push(File::new(file_data, self.sink_output_location.get_file_location(self.starting_block_number, block_number, &self.encoding_type)));
        }

        self.starting_block_number = block_number + 1;
        output
    }

    fn get_an_output_folder_location(&self) -> Location {
        self.sink_output_location.clone()
    }

    fn set_starting_block_number(&mut self, starting_block_number: i64) {
        self.starting_block_number = starting_block_number;
    }
}