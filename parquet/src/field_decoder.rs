use std::borrow::BorrowMut;
use std::fmt::Debug;
use bytes::Bytes;
use parquet::column::writer::ColumnWriter;
use parquet::data_type::ByteArray;
use parquet::file::writer::{SerializedColumnWriter, SerializedRowGroupWriter};

use crate::decoder::{Decoder, FieldSpecification};
use crate::file_buffer::FileBuffer;
use crate::value_store::ValueStore;

pub struct FieldDecoder {
    value_store: ValueStore,
    definition_lvls: Option<Vec<i16>>,
    repetition_lvls: Option<Vec<i16>>,
    field_specification: FieldSpecification,
    flattened_field_name: String
}

impl FieldDecoder {
    pub(crate) fn new(value_store: ValueStore, mut track_definition_lvls: bool, mut track_repetition_lvls: bool, field_specification: FieldSpecification, flattened_field_name: String) -> Self {
        match field_specification {
            FieldSpecification::Required => {}
            FieldSpecification::Optional => track_definition_lvls = true,
            FieldSpecification::Repeated => track_repetition_lvls = true,
            FieldSpecification::Packed => track_repetition_lvls = true,
        };

        FieldDecoder {
            value_store,
            definition_lvls: if track_definition_lvls { Some(Vec::new()) } else { None },
            repetition_lvls: if track_repetition_lvls { Some(Vec::new()) } else { None },
            field_specification,
            flattened_field_name
        }
    }

    pub(crate) fn write_data_to_parquet(&mut self, row_group_writer: &mut SerializedRowGroupWriter<FileBuffer>) {
        let mut serialized_column_writer = row_group_writer.next_column().unwrap().unwrap();

        macro_rules! write_batch {
            ($column_writer_ident:ident, $values_ident:ident) => {
                $column_writer_ident.write_batch(
                    $values_ident,
                    self.definition_lvls.as_ref().map(|lvls| lvls.as_slice()),
                    self.repetition_lvls.as_ref().map(|lvls| lvls.as_slice())
                ).unwrap()
            }
        }

        // macro_rules! write_batch {
        //     ($column_writer_ident:ident, $values_ident:ident) => {
        //         println!("Num values: {}", $values_ident.len());
        //     }
        // }

        match (self.value_store.borrow_mut(), serialized_column_writer.untyped()) {
            (ValueStore::Double(values), ColumnWriter::DoubleColumnWriter(ref mut column_writer)) => {
                write_batch!(column_writer, values)
            }
            (ValueStore::Float(values), ColumnWriter::FloatColumnWriter(ref mut column_writer)) => {
                write_batch!(column_writer, values)
            }
            (ValueStore::Int32(values), ColumnWriter::Int32ColumnWriter(ref mut column_writer)) => {
                write_batch!(column_writer, values)
            }
            (ValueStore::Int64(values), ColumnWriter::Int64ColumnWriter(ref mut column_writer)) => {
                write_batch!(column_writer, values)
            }
            (ValueStore::UInt32(values), ColumnWriter::Int32ColumnWriter(ref mut column_writer)) => {
                write_batch!(column_writer, values)
            }
            (ValueStore::UInt64(values), ColumnWriter::Int64ColumnWriter(ref mut column_writer)) => {
                write_batch!(column_writer, values)
            }
            (ValueStore::SInt32(values), ColumnWriter::Int32ColumnWriter(ref mut column_writer)) => {
                write_batch!(column_writer, values)
            }
            (ValueStore::SInt64(values), ColumnWriter::Int64ColumnWriter(ref mut column_writer)) => {
                write_batch!(column_writer, values)
            }
            (ValueStore::Fixed32(values), ColumnWriter::Int32ColumnWriter(ref mut column_writer)) => {
                write_batch!(column_writer, values)
            }
            (ValueStore::Fixed64(values), ColumnWriter::Int64ColumnWriter(ref mut column_writer)) => {
                write_batch!(column_writer, values)
            }
            (ValueStore::SFixed32(values), ColumnWriter::Int32ColumnWriter(ref mut column_writer)) => {
                write_batch!(column_writer, values)
            }
            (ValueStore::SFixed64(values), ColumnWriter::Int64ColumnWriter(ref mut column_writer)) => {
                write_batch!(column_writer, values)
            }
            (ValueStore::Bool(values), ColumnWriter::BoolColumnWriter(ref mut column_writer)) => {
                write_batch!(column_writer, values)
            }
            (ValueStore::String(values), ColumnWriter::ByteArrayColumnWriter(ref mut column_writer)) => {
                write_batch!(column_writer, values)
            }
            (ValueStore::Bytes(values), ColumnWriter::ByteArrayColumnWriter(ref mut column_writer)) => {
                write_batch!(column_writer, values)
            }
            (ValueStore::Enum(values), ColumnWriter::Int64ColumnWriter(ref mut column_writer)) => {
                write_batch!(column_writer, values)
            },
            _ => unreachable!()
        };

        serialized_column_writer.close().unwrap();
    }

    /// This is triggered when the proto data does not contain a value for a given field.
    pub(crate) fn push_null_or_default_value(&mut self, uncompressed_file_size: &mut usize, current_definition_lvl: i16, last_repetition_lvl: &mut i16) -> Result<(), String> {
        match self.field_specification {
            FieldSpecification::Required => {
                self.value_store.push_default_value();
                // TODO: If tracking def and rep lvls these need to be updated here also
                Ok(())
            },
            FieldSpecification::Optional => self.push_null(uncompressed_file_size, current_definition_lvl, last_repetition_lvl),
            _ => Ok(())
        }
    }

    pub(crate) fn push_null(&mut self, uncompressed_file_size: &mut usize, current_definition_lvl: i16, last_repetition_lvl: &mut i16) -> Result<(), String> {
        todo!()
    }

    pub(crate) fn decode(&mut self, data: &mut &[u8], wire_type: u8, uncompressed_file_size: &mut usize, current_definition_lvl: i16, last_repetition_lvl: &mut i16) -> Result<(), String> {
        match self.field_specification {
            FieldSpecification::Required => {
                self.decode_single_value(data, wire_type, uncompressed_file_size)?;
                if let Some(repetition_lvls) = self.repetition_lvls.as_mut() {
                    repetition_lvls.push(*last_repetition_lvl);
                }

                if let Some(definition_lvls) = self.definition_lvls.as_mut() {
                    definition_lvls.push(current_definition_lvl);
                }
            }
            FieldSpecification::Optional => {
                self.decode_single_value(data, wire_type, uncompressed_file_size)?;
                if let Some(repetition_lvls) = self.repetition_lvls.as_mut() {
                    repetition_lvls.push(*last_repetition_lvl);
                }

                self.definition_lvls.as_mut().unwrap().push(current_definition_lvl+1);
            }
            FieldSpecification::Repeated => {
                self.decode_single_value(data, wire_type, uncompressed_file_size)?;
                self.repetition_lvls.as_mut().unwrap().push(*last_repetition_lvl);
                *last_repetition_lvl += 1;

                if let Some(definition_lvls) = self.definition_lvls.as_mut() {
                    definition_lvls.push(current_definition_lvl);
                }
            }
            FieldSpecification::Packed => {
                return if wire_type == 2 {
                    self.decode_packed(data, uncompressed_file_size, current_definition_lvl, last_repetition_lvl)
                } else {
                    Err("TODO an error message here!!".to_string())
                }
            }
        }

        Ok(())
    }

    pub(crate) fn decode_single_value(&mut self, data: &mut &[u8], wire_type: u8, uncompressed_file_size: &mut usize) -> Result<(), String> {
        // Todo: Add wire_type checks for single value decoding if necessary

        macro_rules! decode_value {
            ($field_type:literal @ $values_ident:ident => $uncompressed_size_delta:expr => $val:ident = $try_read:expr => $insert:expr ) => {
                match $try_read {
                    Some($val) => {
                        *uncompressed_file_size += $uncompressed_size_delta;
                        $values_ident.push($insert);
                    },
                    // TODO: Move $field_type into string literal and also use better method for displaying the bytes!
                    None => return Err(format!("Error reading proto data for column: {}! Field Type: {}, data: {:?}", self.flattened_field_name, $field_type, data)),
                }
            }
        }

        // TODO: Add in good error responses for all possible failure events here, eg during the passing of UInt32 values,
        // TODO: when converting the parsed u32 values to i32 the conversion should be done safely with a clear error
        // TODO: when the u32 value is too large to be able to be converted to i32
        match self.value_store.borrow_mut() {
            ValueStore::Double(values) => {
                decode_value! { "Double" @ values => 64 => b = try_read_8_bytes(data) => f64::from_le_bytes(b) }
            }
            ValueStore::Float(values) => {
                decode_value! { "Float" @ values => 32 => b = try_read_4_bytes(data) => f32::from_le_bytes(b) }
            }
            ValueStore::Int32(values) => {
                decode_value! { "Int32" @ values => 32 => b = i32::from_signed_varint(data) => b }
            }
            ValueStore::Int64(values) => {
                decode_value! { "Int64" @ values => 64 => b = i64::from_signed_varint(data) => b }
            }
            ValueStore::UInt32(values) => {
                decode_value! { "UInt32" @ values => 32 => b = u32::from_signed_varint(data) => b as i32 }
            }
            ValueStore::UInt64(values) => {
                decode_value! { "UInt64" @ values => 64 => b = u64::from_signed_varint(data) => b as i64 }
            }
            ValueStore::SInt32(values) => {
                decode_value! { "SInt32" @ values => 32 => b = u32::from_unsigned_varint(data) => {
                    let (sign, sign_bit) = if b % 2 == 0 { (1i32, 0) } else { (-1i32, 1) };
                    let magnitude = (b / 2) as i32 + sign_bit;
                    sign * magnitude
                } }
            }
            ValueStore::SInt64(values) => {
                decode_value! { "SInt64" @ values => 64 => b = u64::from_unsigned_varint(data) => {
                    let (sign, sign_bit) = if b % 2 == 0 { (1i64, 0) } else { (-1i64, 1) };
                    let magnitude = (b / 2) as i64 + sign_bit;
                    sign * magnitude
                } }
            }
            ValueStore::Fixed32(values) => {
                decode_value! { "Fixed32" @ values => 32 => b = try_read_4_bytes(data) => u32::from_le_bytes(b) as i32 }
            }
            ValueStore::Fixed64(values) => {
                decode_value! { "Fixed64" @ values => 64 => b = try_read_8_bytes(data) => u64::from_le_bytes(b) as i64 }
            }
            ValueStore::SFixed32(values) => {
                decode_value! { "SFixed32" @ values => 32 => b = try_read_4_bytes(data) => i32::from_le_bytes(b) }
            }
            ValueStore::SFixed64(values) => {
                decode_value! { "SFixed64" @ values => 64 => b = try_read_8_bytes(data) => i64::from_le_bytes(b) }
            }
            ValueStore::Bool(values) => {
                decode_value! { "Bool" @ values => 8 => b = usize::from_unsigned_varint(data) => b != 0 }
            }
            ValueStore::String(values) => {
                decode_value! { "String" @ values => b.len() => b = read_string(data) => b }
            }
            ValueStore::Bytes(values) => {
                decode_value! { "Bytes" @ values => b.len() => b = read_bytes(data) => b }
            }
            ValueStore::Enum(values) => {
                decode_value! { "Enum" @ values => 64 => b = i64::from_signed_varint(data) => b }
            }
        };

        Ok(())
    }

    fn decode_packed(&mut self, data: &mut &[u8], uncompressed_file_size: &mut usize, current_definition_lvl: i16, last_repetition_lvl: &mut i16) -> Result<(), String> {
        let packed_values_data_size = match usize::from_unsigned_varint(data) {
            Some(len) => len,
            None => {
                return Err(format!("Error reading encoded packed values data size when decoding proto data for packed column: {}! Unprocessed proto data: {:?}", self.flattened_field_name, data));
            }
        };

        if data.len() < packed_values_data_size {
            return Err(format!("Error with insufficient data for reading proto data for column: {}! Size of packed values data: {}B, Size of unprocessed proto data; {}B, Unprocessed proto data: {:?}", self.flattened_field_name, packed_values_data_size, data.len(), data))
        }

        let mut packed_values_data = &data[..packed_values_data_size];
        *data = &data[packed_values_data_size..];

        macro_rules! decode_packed_values {
            ($field_type:literal @ $values_ident:ident => $uncompressed_size_delta:expr => $val:ident = $try_read:expr => $insert:expr ) => {
                let mut values_read = 0;
                loop {
                    if packed_values_data.is_empty() {
                        break;
                    }

                    match $try_read {
                        Some($val) => {
                            $values_ident.push($insert);
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
                decode_packed_values! { "Double" @ values => 64 => b = try_read_8_bytes(&mut packed_values_data) => f64::from_le_bytes(b) }
            }
            ValueStore::Float(values) => {
                decode_packed_values! { "Float" @ values => 32 => b = try_read_4_bytes(&mut packed_values_data) => f32::from_le_bytes(b) }
            }
            ValueStore::Int32(values) => {
                decode_packed_values! { "Int32" @ values => 32 => b = i32::from_signed_varint(&mut packed_values_data) => b }
            }
            ValueStore::Int64(values) => {
                decode_packed_values! { "Int64" @ values => 64 => b = i64::from_signed_varint(&mut packed_values_data) => b }
            }
            ValueStore::UInt32(values) => {
                decode_packed_values! { "UInt32" @ values => 32 => b = u32::from_signed_varint(&mut packed_values_data) => b as i32 }
            }
            ValueStore::UInt64(values) => {
                decode_packed_values! { "UInt64" @ values => 64 => b = u64::from_signed_varint(&mut packed_values_data) => b as i64 }
            }
            ValueStore::SInt32(values) => {
                decode_packed_values! { "SInt32" @ values => 32 => b = u32::from_signed_varint(&mut packed_values_data) => {
                    let (sign, sign_bit) = if b % 2 == 0 { (1i32, 0) } else { (-1i32, 1) };
                    let magnitude = (b / 2) as i32 + sign_bit;
                    sign * magnitude
                } }
            }
            ValueStore::SInt64(values) => {
                decode_packed_values! { "SInt64" @ values => 64 => b = u64::from_signed_varint(&mut packed_values_data) => {
                    let (sign, sign_bit) = if b % 2 == 0 { (1i64, 0) } else { (-1i64, 1) };
                    let magnitude = (b / 2) as i64 + sign_bit;
                    sign * magnitude
                } }
            }
            ValueStore::Fixed32(values) => {
                decode_packed_values! { "Fixed32" @ values => 32 => b = try_read_4_bytes(&mut packed_values_data) => u32::from_le_bytes(b) as i32 }
            }
            ValueStore::Fixed64(values) => {
                decode_packed_values! { "Fixed64" @ values => 64 => b = try_read_8_bytes(&mut packed_values_data) => u64::from_le_bytes(b) as i64 }
            }
            ValueStore::SFixed32(values) => {
                decode_packed_values! { "SFixed32" @ values => 32 => b = try_read_4_bytes(&mut packed_values_data) => i32::from_le_bytes(b) }
            }
            ValueStore::SFixed64(values) => {
                decode_packed_values! { "SFixed64" @ values => 64 => b = try_read_8_bytes(&mut packed_values_data) => i64::from_le_bytes(b) }
            }
            ValueStore::Bool(values) => {
                decode_packed_values! { "Bool" @ values => 8 => b = u8::from_unsigned_varint(&mut packed_values_data) => b != 0 }
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
                definition_lvls.extend(vec![current_definition_lvl; values_read]);
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

fn read_bytes(data: &mut &[u8]) -> Option<ByteArray>
{
    let original = *data;
    let len = usize::from_unsigned_varint(data)?;
    if len > data.len() {
        *data = original;
        return None;
    }
    let (str_data, remainder) = data.split_at(len);
    *data = remainder;
    Some(ByteArray::from(str_data.to_vec()))
}

fn read_string(data: &mut &[u8]) -> Option<ByteArray>
{
    let original = *data;
    let len = usize::from_unsigned_varint(data)?;

    if len > data.len() {
        *data = original;
        return None;
    }
    let (str_data, remainder) = data.split_at(len);
    *data = remainder;
    Some(ByteArray::from(String::from_utf8_lossy(str_data).to_string().as_str()))
}