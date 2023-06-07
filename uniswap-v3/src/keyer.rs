use substreams::Hex;

pub fn get_data_source_key(address: &Vec<u8>) -> String {
    format!("DataSource:{}", Hex(address).to_string())
}
