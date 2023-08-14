use std::sync::Arc;
use parquet::basic::{Compression, ConvertedType, LogicalType, Repetition};
use parquet::file::properties::{WriterProperties, WriterPropertiesPtr};
use parquet::schema::types::{GroupTypeBuilder, PrimitiveTypeBuilder, TypePtr};
use derives::proto_structure_info::FieldType;

pub(in crate::streaming_fast::file_sinks) struct ParquetSchemaBuilder {
    subgroup_fields: Vec<Vec<TypePtr>>,
    hierarchy_trace: Vec<String>,
    current_id: i32,
    type_name: String
}

impl ParquetSchemaBuilder {
    pub(in crate::streaming_fast::file_sinks) fn new(type_name: String) -> Self {
        ParquetSchemaBuilder {
            subgroup_fields: vec![vec![Arc::new(PrimitiveTypeBuilder::new("block_number", parquet::basic::Type::INT64).with_id(1).with_repetition(Repetition::REQUIRED).with_logical_type(Some(LogicalType::Integer{ bit_width: 64, is_signed: false })).build().unwrap())]],
            hierarchy_trace: vec![],
            current_id: 1,
            type_name,
        }
    }

    pub(in crate::streaming_fast::file_sinks) fn start_building_sub_group(&mut self, field_name: String) {
        self.hierarchy_trace.push(field_name);
        self.subgroup_fields.push(Vec::new());
    }

    pub(in crate::streaming_fast::file_sinks) fn finish_building_sub_group(&mut self, repetition: Repetition) {
        let field_name = self.hierarchy_trace.pop().unwrap();
        let mut group_fields = self.subgroup_fields.pop().unwrap();
        let new_field = if repetition == Repetition::REPEATED {
            GroupTypeBuilder::new(&field_name).with_converted_type(ConvertedType::LIST).with_repetition(Repetition::REQUIRED)
                .with_fields(&mut vec![Arc::new(GroupTypeBuilder::new(&field_name).with_fields(&mut group_fields).with_repetition(repetition).build().unwrap())])
                .build().unwrap()
        } else {
            GroupTypeBuilder::new(&field_name).with_fields(&mut group_fields).with_repetition(repetition).build().unwrap()
        };

        self.subgroup_fields.last_mut().unwrap().push(Arc::new(new_field));
    }

    /// Returns the a parquet file schema and it's corresponding write properties in the form => (parquet_schema, writer_properties)
    pub(in crate::streaming_fast::file_sinks) fn compile(mut self) -> (TypePtr, WriterPropertiesPtr) {
        assert!(self.hierarchy_trace.len()==0 && self.subgroup_fields.len()==1);

        let group_builder = GroupTypeBuilder::new(&self.type_name);
        let mut group_fields = self.subgroup_fields.pop().unwrap();

        let parquet_schema = group_builder.with_fields(&mut group_fields).build().unwrap();

        let writer_properties = Arc::new(WriterProperties::builder().set_compression(Compression::SNAPPY).build());

        (Arc::new(parquet_schema), writer_properties)
    }

    pub(in crate::streaming_fast::file_sinks) fn get_flattened_field_name(&self, field_name: &str) -> String {
        if self.hierarchy_trace.is_empty() {
            field_name.to_string()
        } else {
            format!("{}__{}", self.hierarchy_trace.join("__"), field_name)
        }
    }

    pub(in crate::streaming_fast::file_sinks) fn add_column_info(&mut self, field_name: &str, field_type: FieldType, repetition: Repetition) {
        macro_rules! add_field {
            ($physical_type:ident @ $logical_type:expr) => {
                if repetition == Repetition::REPEATED {
                    self.subgroup_fields.last_mut().unwrap().push(Arc::new(GroupTypeBuilder::new(&field_name).with_converted_type(ConvertedType::LIST).with_repetition(Repetition::REQUIRED)
                        .with_fields(&mut vec![Arc::new(PrimitiveTypeBuilder::new(field_name, parquet::basic::Type::$physical_type).with_id(self.current_id)
                        .with_repetition(repetition).with_logical_type(Some($logical_type)).build().unwrap())]).build().unwrap()));
                } else {
                    self.subgroup_fields.last_mut().unwrap().push(Arc::new(PrimitiveTypeBuilder::new(field_name, parquet::basic::Type::$physical_type)
                        .with_id(self.current_id).with_repetition(repetition).with_logical_type(Some($logical_type)).build().unwrap()));
                }
            };
            ($physical_type:ident) => {
                if repetition == Repetition::REPEATED {
                    self.subgroup_fields.last_mut().unwrap().push(Arc::new(GroupTypeBuilder::new(&field_name).with_converted_type(ConvertedType::LIST).with_repetition(Repetition::REQUIRED)
                        .with_fields(&mut vec![Arc::new(PrimitiveTypeBuilder::new(field_name, parquet::basic::Type::$physical_type)
                        .with_id(self.current_id).with_repetition(repetition).build().unwrap())]).build().unwrap()));
                } else {
                    self.subgroup_fields.last_mut().unwrap().push(Arc::new(PrimitiveTypeBuilder::new(field_name, parquet::basic::Type::$physical_type)
                        .with_id(self.current_id).with_repetition(repetition).build().unwrap()))
                }
            };
        }

        self.current_id += 1;

        match field_type {
            FieldType::Double => add_field!(DOUBLE),
            FieldType::Float => add_field!(FLOAT),
            FieldType::Int64 => add_field!(INT64),
            FieldType::Uint64 => add_field!(INT64 @ LogicalType::Integer{ bit_width: 64, is_signed: false }),
            FieldType::Int32 => add_field!(INT32),
            FieldType::Fixed64 => add_field!(INT64 @ LogicalType::Integer{ bit_width: 64, is_signed: false }),
            FieldType::Fixed32 => add_field!(INT32 @ LogicalType::Integer{ bit_width: 32, is_signed: false }),
            FieldType::Bool => add_field!(BOOLEAN),
            FieldType::String => add_field!(BYTE_ARRAY @ LogicalType::String),
            FieldType::Bytes => add_field!(BYTE_ARRAY),
            FieldType::Uint32 => add_field!(INT32 @ LogicalType::Integer{ bit_width: 32, is_signed: false }),
            FieldType::Enum(_) => add_field!(BYTE_ARRAY @ LogicalType::Enum),
            FieldType::Sfixed32 => add_field!(INT32),
            FieldType::Sfixed64 => add_field!(INT64),
            FieldType::Sint32 => add_field!(INT32),
            FieldType::Sint64 => add_field!(INT64),
            FieldType::Message(_) => unreachable!()
        }
    }
}