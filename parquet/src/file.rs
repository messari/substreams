use std::path::PathBuf;

pub(crate) struct File {
    file_data: Vec<u8>,
    output_location: Location
}

impl File {
    pub(crate) fn new(file_data: Vec<u8>, output_location: Location) -> File {
        File {
            file_data,
            output_location,
        }
    }
}

pub(crate) enum Location {
    DataWarehouse(PathBuf),
    Local(PathBuf)
}

impl Location {
    pub(crate) fn new(location_type: LocationType, path: PathBuf) -> Location {
        match location_type {
            LocationType::DataWarehouse => Location::DataWarehouse(path),
            LocationType::Local => Location::Local(path)
        }
    }

    pub(crate) async fn save(self) {
        match self {
            Location::DataWarehouse(filepath) => {
                // TODO: Add AWS code for a file upload
            }
            Location::Local(filepath) => {
                // Add tokio code for locally saving file
            }
        }
    }

    pub(crate) fn get_file_location(&self, first_block_number: i64, last_block_number: &i64) -> Location {
        let filename = format!("{}_{}.parquet", first_block_number, last_block_number);

        match self {
            Location::DataWarehouse(base_path) => Location::DataWarehouse(base_path.join(filename)),
            Location::Local(base_path) => Location::Local(base_path.join(filename))
        }
    }
}

impl Location {
    pub(crate) fn data_warehouse(path: PathBuf) -> Location {
        Location::DataWarehouse(path)
    }

    pub(crate) fn local(path: PathBuf) -> Location {
        Location::Local(path)
    }
}

pub(crate) enum LocationType {
    DataWarehouse,
    Local
}