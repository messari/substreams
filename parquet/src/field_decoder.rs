use std::borrow::BorrowMut;
use std::fmt::Debug;
use bytes::Bytes;
use parquet::column::writer::ColumnWriter;
use parquet::file::writer::{SerializedColumnWriter, SerializedRowGroupWriter};

use crate::decoder::Decoder;
use crate::file_buffer::FileBuffer;
use crate::sink_error::SinkError;
use crate::value_store::ValueStore;

pub struct FieldDecoder {
    value_store: ValueStore,
    definition_lvls: Option<Vec<i16>>,
    repetition_lvls: Option<Vec<i16>>,
    is_repeated: bool,
    is_packed: bool,
    is_optional: bool,
    flattened_field_name: String
}

impl FieldDecoder {
    pub(crate) fn new(value_store: ValueStore, track_definition_lvls: bool, track_repetition_lvls: bool, is_repeated: bool, is_packed: bool, is_optional: bool, flattened_field_name: String) -> Self {
        FieldDecoder {
            value_store,
            definition_lvls: if track_definition_lvls { Some(Vec::new()) } else { None },
            repetition_lvls: if track_repetition_lvls { Some(Vec::new()) } else { None },
            is_repeated,
            is_packed,
            is_optional,
            flattened_field_name
        }
    }

    pub(crate) fn write_data_to_parquet(&mut self, row_group_writer: &mut SerializedRowGroupWriter<FileBuffer>) {
        let serialized_column_writer = row_group_writer.next_column().unwrap().unwrap();

        macro_rules! write_batch {
            () => {
                column_writer.write_batch(
                    values,
                    self.definition_lvls.as_ref().map(|lvls| lvls.as_slice()),
                    self.repetition_lvls.as_ref().map(|lvls| lvls.as_slice())
                ).unwrap();
            }
        }

        match (self.value_store.borrow_mut(), serialized_column_writer) {
            (ValueStore::Double(values), ColumnWriter::DoubleColumnWriter(ref mut column_writer)) => {
                write_batch!()
            }
            (ValueStore::Float(values), ColumnWriter::FloatColumnWriter(ref mut column_writer)) => {
                write_batch!()
            }
            (ValueStore::Int32(values), ColumnWriter::Int32ColumnWriter(ref mut column_writer)) => {
                write_batch!()
            }
            (ValueStore::Int64(values), ColumnWriter::Int64ColumnWriter(ref mut column_writer)) => {
                write_batch!()
            }
            (ValueStore::UInt32(values), ColumnWriter::Int32ColumnWriter(ref mut column_writer)) => {
                // TODO: Fix this type mismatch!
                write_batch!()
            }
            (ValueStore::UInt64(values), ColumnWriter::Int64ColumnWriter(ref mut column_writer)) => {
                // TODO: Fix this type mismatch!
                write_batch!()
            }
            (ValueStore::SInt32(values), ColumnWriter::Int32ColumnWriter(ref mut column_writer)) => {
                write_batch!()
            }
            (ValueStore::SInt64(values), ColumnWriter::Int64ColumnWriter(ref mut column_writer)) => {
                write_batch!()
            }
            (ValueStore::Fixed32(values), ColumnWriter::Int32ColumnWriter(ref mut column_writer)) => {
                // TODO: Fix this type mismatch!
                write_batch!()
            }
            (ValueStore::Fixed64(values), ColumnWriter::Int64ColumnWriter(ref mut column_writer)) => {
                // TODO: Fix this type mismatch!
                write_batch!()
            }
            (ValueStore::SFixed32(values), ColumnWriter::Int32ColumnWriter(ref mut column_writer)) => {
                write_batch!()
            }
            (ValueStore::SFixed64(values), ColumnWriter::Int64ColumnWriter(ref mut column_writer)) => {
                write_batch!()
            }
            (ValueStore::Bool(values), ColumnWriter::BoolColumnWriter(ref mut column_writer)) => {
                write_batch!()
            }
            (ValueStore::String(values), ColumnWriter::ByteArrayColumnWriter(ref mut column_writer)) => {
                // TODO: Maybe we should store the strings as byte arrays in the valueStore so we don't have to convert back and forth here?
                write_batch!()
            }
            (ValueStore::Bytes(values), ColumnWriter::ByteArrayColumnWriter(ref mut column_writer)) => {
                write_batch!()
            }
            (ValueStore::Enum(values), ColumnWriter::Int64ColumnWriter(ref mut column_writer)) => {
                write_batch!()
            },
            _ => unreachable!()
        }

        // TODO: Double check that this properly closes the column writer
        serialized_column_writer.close().unwrap();
    }

    pub(crate) fn decode(&mut self, data: &mut &[u8], wire_type: u8, uncompressed_file_size: &mut usize, current_definition_lvl: i16, last_repetition_lvl: &mut i16) -> Result<(), String> {
        if self.is_packed {
            return if wire_type == 2 {
                self.decode_packed(data, uncompressed_file_size, current_definition_lvl, last_repetition_lvl)
            } else {
                Err("TODO an error message here!!".to_string())
            }
        }

        macro_rules! decode_value {
            ($field_type:str => $uncompressed_size_delta:expr @ $val:ident = $try_read:expr => $insert:expr ) => {
                match $try_read {
                    Some($val) => {
                        values.push($insert);
                        *uncompressed_file_size += $uncompressed_size_delta;
                    },
                    // TODO: Move $field_type into string literal and also use better method for displaying the bytes!
                    None => return Err(format!("Error reading proto data for column: {}! Field Type: {}, data: {:?}", self.flattened_field_name, $field_type, data)),
                }
            }
        }

        // TODO: Make sure the wire_type matches the given field_type - if not then throw and error
        match self.value_store.borrow_mut() {
            ValueStore::Double(values) => {
                decode_value! { Double => 64 @ b = try_read_8_bytes(&mut array) => f64::from_le_bytes(b) }
            }
            ValueStore::Float(values) => {
                decode_value! { Float => 32 @ b = try_read_4_bytes(&mut array) => f32::from_le_bytes(b) }
            }
            ValueStore::Int32(values) => {
                decode_value! { Int32 => 32 @ b = i32::from_signed_varint(&mut array) => b }
            }
            ValueStore::Int64(values) => {
                decode_value! { Int64 => 64 @ b = i64::from_signed_varint(&mut array) => b }
            }
            ValueStore::UInt32(values) => {
                decode_value! { UInt32 => 32 @ b = u32::from_signed_varint(&mut array) => b }
            }
            ValueStore::UInt64(values) => {
                decode_value! { UInt64 => 64 @ b = u64::from_signed_varint(&mut array) => b }
            }
            ValueStore::SInt32(values) => {
                decode_value! { SInt32 => 32 @ b = u32::from_unsigned_varint(data)(&mut array) => {
                    let (sign, sign_bit) = if b % 2 == 0 { (1i32, 0) } else { (-1i32, 1) };
                    let magnitude = (b / 2) as i32 + sign_bit;
                    sign * magnitude
                } }
            }
            ValueStore::SInt64(values) => {
                decode_value! { SInt64 => 64 @ b = u64::from_unsigned_varint(&mut array) => {
                    let (sign, sign_bit) = if b % 2 == 0 { (1i64, 0) } else { (-1i64, 1) };
                    let magnitude = (b / 2) as i64 + sign_bit;
                    sign * magnitude
                } }
            }
            ValueStore::Fixed32(values) => {
                decode_value! { Fixed32 => 32 @ b = try_read_4_bytes(&mut array) => u32::from_le_bytes(b) }
            }
            ValueStore::Fixed64(values) => {
                decode_value! { Fixed64 => 64 @ b = try_read_8_bytes(&mut array) => u64::from_le_bytes(b) }
            }
            ValueStore::SFixed32(values) => {
                decode_value! { SFixed32 => 32 @ b = try_read_4_bytes(&mut array) => i32::from_le_bytes(b) }
            }
            ValueStore::SFixed64(values) => {
                decode_value! { SFixed64 => 64 @ b = try_read_8_bytes(&mut array) => i64::from_le_bytes(b) }
            }
            ValueStore::Bool(values) => {
                decode_value! { Bool => 8 @ b = usize::from_unsigned_varint(data) => b != 0 }
            }
            ValueStore::String(values) => {
                decode_value! { String => b.as_bytes().len() @ b = read_string(data) => b != 0 }
            }
            ValueStore::Bytes(values) => {
                decode_value! { Bytes @ b.len() = read_bytes(data) => b != 0 }
            }
            ValueStore::Enum(values) => {
                decode_value! { Enum => 64 @ b = i64::from_signed_varint(data) => b != 0 }
            }
        };

        if let Some(repetition_lvls) = self.repetition_lvls.as_mut() {
            repetition_lvls.push(*last_repetition_lvl);
            if self.is_repeated {
                *last_repetition_lvl += 1;
            }
        }

        if let Some(definition_lvls) = self.definition_lvls.as_mut() {
            if self.is_optional {
                definition_lvls.push(current_definition_lvl+1);
            } else {
                definition_lvls.push(current_definition_lvl);
            }
        }

        Ok(())
    }

    fn decode_packed(&mut self, data: &mut &[u8], uncompressed_file_size: &mut usize, current_definition_lvl: i16, last_repetition_lvl: &mut i16) -> Result<(), String> {
        let length = match usize::from_unsigned_varint(data) {
            Some(len) => len,
            None => {
                return Err(format!("Error reading encoded item length when decoding proto data for packed column: {}! data: {:?}", self.flattened_field_name, $field_type, values_read, packed_values_data));
            }
        };

        if data.len() < length {
            return Err(format!("Error with insufficient data for reading proto data for column: {}! Field Type: {}, Packed value index: {}, data: {:?}", self.flattened_field_name, $field_type, values_read, packed_values_data))
        }

        let mut packed_values_data = &data[..length];
        *data = &data[length..];

        macro_rules! decode_packed_values {
            ($variant:ident => $uncompressed_size_delta:expr @ $val:ident = $try_read:expr => $insert:expr ) => {
                let mut values_read = 0;
                loop {
                    if packed_values_data.is_empty() {
                        break;
                    }

                    match $try_read {
                        Some($val) => {
                            values.push($insert);
                            *uncompressed_file_size += $uncompressed_size_delta;
                            values_read += 1;
                        },
                        None => return Err(format!("Error reading proto data for column: {}! Field Type: {}, Packed value index: {}, data: {:?}", self.flattened_field_name, $field_type, values_read, packed_values_data)),
                    }
                }
                values_read
            };
        }

        let values_read = match self.value_store.borrow_mut() {
            ValueStore::Double(values) => {
                decode_packed_values! { Double => 64 @ b = try_read_8_bytes(&mut packed_values_data) => f64::from_le_bytes(b) }
            }
            ValueStore::Float(values) => {
                decode_packed_values! { Float => 32 @ b = try_read_4_bytes(&mut packed_values_data) => f32::from_le_bytes(b) }
            }
            ValueStore::Int32(values) => {
                decode_packed_values! { Int32 => 32 @ b = i32::from_signed_varint(&mut array) => b }
            }
            ValueStore::Int64(values) => {
                decode_packed_values! { Int64 => 64 @ b = i64::from_signed_varint(&mut array) => b }
            }
            ValueStore::UInt32(values) => {
                decode_packed_values! { UInt32 => 32 @ b = u32::from_signed_varint(&mut array) => b }
            }
            ValueStore::UInt64(values) => {
                decode_packed_values! { UInt64 => 64 @ b = u64::from_signed_varint(&mut array) => b }
            }
            ValueStore::SInt32(values) => {
                decode_packed_values! { SInt32 => 32 @ b = u32::from_signed_varint(&mut array) => {
                    let (sign, sign_bit) = if b % 2 == 0 { (1i32, 0) } else { (-1i32, 1) };
                    let magnitude = (b / 2) as i32 + sign_bit;
                    sign * magnitude
                } }
            }
            ValueStore::SInt64(values) => {
                decode_packed_values! { SInt64 => 64 @ b = u64::from_signed_varint(&mut array) => {
                    let (sign, sign_bit) = if b % 2 == 0 { (1i64, 0) } else { (-1i64, 1) };
                    let magnitude = (b / 2) as i64 + sign_bit;
                    sign * magnitude
                } }
            }
            ValueStore::Fixed32(values) => {
                decode_packed_values! { Fixed32 => 32 @ b = try_read_4_bytes(&mut array) => u32::from_le_bytes(b) }
            }
            ValueStore::Fixed64(values) => {
                decode_packed_values! { Fixed64 => 64 @ b = try_read_8_bytes(&mut array) => u64::from_le_bytes(b) }
            }
            ValueStore::SFixed32(values) => {
                decode_packed_values! { SFixed32 => 32 @ b = try_read_4_bytes(&mut array) => i32::from_le_bytes(b) }
            }
            ValueStore::SFixed64(values) => {
                decode_packed_values! { SFixed64 => 64 @ b = try_read_8_bytes(&mut array) => i64::from_le_bytes(b) }
            }
            ValueStore::Bool(values) => {
                decode_packed_values! { Bool => 8 @ b = u8::from_unsigned_varint(&mut array) => b != 0 }
            }
            _ => panic!("Non-scalar type was handled as packed"),
        };

        if values_read>0 {
            if let Some(repetition_lvls) = self.repetition_lvls.as_mut() {
                repetition_lvls.push(*last_repetition_lvl);
                *last_repetition_lvl += 1;
                repetition_lvls.extend(vec![*last_repetition_lvl; values_read-1]);
            }

            if let Some(definition_lvls) = self.definition_lvls.as_mut() {
                if self.is_optional {
                    definition_lvls.extend(vec![current_definition_lvl+1; values_read]);
                } else {
                    definition_lvls.extend(vec![current_definition_lvl; values_read]);
                }
            }
        } else {
            if let Some(definition_lvls) = self.definition_lvls.as_mut() {
                // TODO: Figure out what to add here..
            }
        }

        Ok(())
    }
}


trait FromUnsignedVarint: Sized
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


fn try_read_8_bytes(data: &mut &[u8]) -> Option<[u8; 8]>
{
    if data.len() < 8 {
        return None;
    }

    match (data[..8]).try_into() {
        Ok(v) => {
            *data = &data[8..];
            Some(v)
        }
        Err(_) => None,
    }
}

fn try_read_4_bytes(data: &mut &[u8]) -> Option<[u8; 4]>
{
    if data.len() < 4 {
        return None;
    }

    match (data[..4]).try_into() {
        Ok(v) => {
            *data = &data[4..];
            Some(v)
        }
        Err(_) => None,
    }
}

trait FromSignedVarint: Sized
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

fn read_bytes(data: &mut &[u8]) -> Option<Bytes>
{
    let original = *data;
    let len = usize::from_unsigned_varint(data)?;
    if len > data.len() {
        *data = original;
        return None;
    }
    let (str_data, remainder) = data.split_at(len);
    *data = remainder;
    Some(Bytes::copy_from_slice(str_data))
}