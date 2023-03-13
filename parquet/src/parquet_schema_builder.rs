use std::sync::Arc;
use parquet::basic::{LogicalType, Repetition};
use parquet::file::properties::{WriterProperties, WriterPropertiesPtr};
use parquet::file::writer::SerializedFileWriter;
use parquet::schema::types::{GroupTypeBuilder, PrimitiveTypeBuilder, TypePtr};
use prost_types::field_descriptor_proto::Type;
use crate::decoder::FieldSpecification;
use crate::file_buffer::FileBuffer;

pub(crate) struct ParquetSchemaBuilder {
    subgroup_fields: Vec<Vec<TypePtr>>,
    hierarchy_trace: Vec<String>,
    current_id: i32,
    type_name: String
}

impl ParquetSchemaBuilder {
    pub(crate) fn new(type_name: &str) -> Self {
        ParquetSchemaBuilder {
            subgroup_fields: vec![vec![Arc::new(PrimitiveTypeBuilder::new("block_number", parquet::basic::Type::INT64).with_id(1).with_repetition(Repetition::REQUIRED).with_logical_type(Some(LogicalType::Integer{ bit_width: 64, is_signed: false })).build().unwrap())]],
            hierarchy_trace: vec![],
            current_id: 1,
            type_name: type_name.to_string(),
        }
    }

    pub(crate) fn start_building_sub_group(&mut self, field_name: &str) {
        self.hierarchy_trace.push(field_name.to_string());
        self.subgroup_fields.push(Vec::new());
    }

    pub(crate) fn finish_building_sub_group(&mut self) {
        let field_name = self.hierarchy_trace.pop().unwrap();
        let group_builder = GroupTypeBuilder::new(&field_name);
        let mut group_fields = self.subgroup_fields.pop().unwrap();
        let new_field = group_builder.with_fields(&mut group_fields).build().unwrap();

        self.subgroup_fields.last_mut().unwrap().push(Arc::new(new_field));
    }

    /// Returns the a parquet file schema and it's corresponding write properties in the form => (parquet_schema, writer_properties)
    pub(crate) fn compile(mut self) -> (TypePtr, WriterPropertiesPtr) {
        assert!(self.hierarchy_trace.len()==0 && self.subgroup_fields.len()==1);

        let group_builder = GroupTypeBuilder::new(&self.type_name);
        let mut group_fields = self.subgroup_fields.pop().unwrap();

        let parquet_schema = group_builder.with_fields(&mut group_fields).build().unwrap();

        // For now we will use the default, however later on we can use a non default setup to optimise for data storage
        let writer_properties = Arc::new(WriterProperties::builder().build());

        (Arc::new(parquet_schema), writer_properties)
    }

    fn get_flattened_field_name(&self, field_name: &str) -> String {
        if self.hierarchy_trace.is_empty() {
            field_name.to_string()
        } else {
            format!("{}__{}", self.hierarchy_trace.join("__"), field_name)
        }
    }

    pub(crate) fn add_column_info(&mut self, field_name: &str, field_type: Type, field_specification: &FieldSpecification) -> String {
        let flattened_field_name = self.get_flattened_field_name(field_name);

        macro_rules! add_field {
            ($physical_type:ident @ $logical_type:expr) => {
                self.subgroup_fields.last_mut().unwrap().push(Arc::new(PrimitiveTypeBuilder::new(&flattened_field_name, parquet::basic::Type::$physical_type)
                    .with_id(self.current_id).with_repetition(field_specification.get_repetition()).with_logical_type(Some($logical_type)).build().unwrap()))
            };
            ($physical_type:ident) => {
                self.subgroup_fields.last_mut().unwrap().push(Arc::new(PrimitiveTypeBuilder::new(&flattened_field_name, parquet::basic::Type::$physical_type)
                    .with_id(self.current_id).with_repetition(field_specification.get_repetition()).build().unwrap()))
            };
        }

        match field_type {
            Type::Double => add_field!(DOUBLE),
            Type::Float => add_field!(FLOAT),
            Type::Int64 => add_field!(INT64),
            Type::Uint64 => add_field!(INT64 @ LogicalType::Integer{ bit_width: 64, is_signed: false }),
            Type::Int32 => add_field!(INT32),
            Type::Fixed64 => add_field!(INT64 @ LogicalType::Integer{ bit_width: 64, is_signed: false }),
            Type::Fixed32 => add_field!(INT32 @ LogicalType::Integer{ bit_width: 32, is_signed: false }),
            Type::Bool => add_field!(BOOLEAN),
            Type::String => add_field!(BYTE_ARRAY @ LogicalType::String),
            Type::Group => unreachable!(),
            Type::Message => unreachable!(),
            Type::Bytes => add_field!(BYTE_ARRAY),
            Type::Uint32 => add_field!(INT32 @ LogicalType::Integer{ bit_width: 32, is_signed: false }),
            Type::Enum => add_field!(INT32 @ LogicalType::Enum), // TODO: Should be changed to BYTE_ARRAY type here..
            Type::Sfixed32 => add_field!(INT32),
            Type::Sfixed64 => add_field!(INT64),
            Type::Sint32 => add_field!(INT32),
            Type::Sint64 => add_field!(INT64)
        }

        self.current_id += 1;

        flattened_field_name
    }
}