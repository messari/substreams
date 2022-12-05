use substreams::scalar::{BigInt};
use std::str::FromStr;

// Convert String to BigInt type
pub fn convert_string_to_bigint(string: &String) -> BigInt {
    BigInt::from_str(string).unwrap()
}

// Convert BigInt array to negative BigInt array
pub fn negative_bi_array(input: Vec<String>) -> Vec<String> {
    let mut output = vec![];
    for i in input {
        let negative_bi = convert_string_to_bigint(&i).neg();
        output.push(negative_bi.to_string());
    }
    output
}

// Get delta between two BigInt arrays
pub fn get_delta_bi_array(input1: &Vec<String>, input2: &Vec<String>) -> Vec<String> {
    let mut output = vec![];
    for (i, j) in input1.iter().zip(input2.iter()) {
        let delta_bi = convert_string_to_bigint(i) - convert_string_to_bigint(j);
        output.push(delta_bi.to_string());
    }
    output
}