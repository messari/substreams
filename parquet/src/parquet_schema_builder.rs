use std::sync::Arc;
use parquet::basic::LogicalType;
use parquet::file::properties::WriterProperties;
use parquet::file::writer::SerializedFileWriter;
use parquet::schema::types::{GroupTypeBuilder, PrimitiveTypeBuilder, TypePtr};
use prost_types::field_descriptor_proto::Type;
use crate::file_buffer::FileBuffer;

pub(crate) struct ParquetSchemaBuilder<'a> {
    group_builders: Vec<GroupTypeBuilder<'a>>,
    subgroup_fields: Vec<Vec<TypePtr>>,
    hierarchy_trace: Vec<String>,
    current_id: i32
}

impl ParquetSchemaBuilder {
    pub(crate) fn new(type_name: &str) -> Self {
        ParquetSchemaBuilder {
            group_builders: vec![GroupTypeBuilder::new(type_name)],
            subgroup_fields: vec![vec![Arc::new(PrimitiveTypeBuilder::new("block_number", parquet::basic::Type::INT64).with_id(1).build().unwrap())]],
            hierarchy_trace: vec![],
            current_id: 1,
        }
    }

    pub(crate) fn start_building_sub_group(&mut self, field_name: &str) {
        self.hierarchy_trace.push(field_name.to_string());
        self.group_builders.push(GroupTypeBuilder::new(field_name));
    }

    pub(crate) fn finish_building_sub_group(&mut self) {
        let group_builder = self.group_builders.pop().unwrap();
        let mut group_fields = self.subgroup_fields.pop().unwrap();
        let new_field = group_builder.with_fields(&mut group_fields).build().unwrap();

        self.subgroup_fields.last().unwrap().push(Arc::new(new_field));
        self.hierarchy_trace.pop().unwrap();
    }

    /// Returns the a parquet file writer and it's corresponding file buffer in the form => (file_writer, file_buffer)
    pub(crate) fn compile(mut self) -> (SerializedFileWriter<FileBuffer>, FileBuffer) {
        assert!(self.group_builders.len()==1 && self.subgroup_fields.len()==1);
        let group_builder = self.group_builders.pop().unwrap();
        group_builder.with_fields(self.subgroup_fields.last_mut().unwrap());

        let schema = group_builder.build().unwrap();

        let file_buffer = FileBuffer::new();
        let file_writer = SerializedFileWriter::new(file_buffer.clone(), Arc::new(schema), Arc::new(WriterProperties::builder().build())).unwrap();

        (file_writer, file_buffer)
    }

    pub(crate) fn get_flattened_field_name(&self, field_name: &str) -> String {
        if self.hierarchy_trace.is_empty() {
            field_name.to_string()
        } else {
            format!("{}__{}", self.hierarchy_trace.join("__"), field_name)
        }
    }

    pub(crate) fn add_column_info(&mut self, field_name: &str, field_type: Type) {
        let flattened_column_name = self.get_flattened_colummn_name(field_name);

        match field_type {
            Type::Double => {
                self.subgroup_fields.last_mut().unwrap().push(Arc::new(PrimitiveTypeBuilder::new(&flattened_column_name, parquet::basic::Type::DOUBLE).with_id(self.current_id).build().unwrap()));
            }
            Type::Float => {
                self.subgroup_fields.last_mut().unwrap().push(Arc::new(PrimitiveTypeBuilder::new(&flattened_column_name, parquet::basic::Type::FLOAT).with_id(self.current_id).build().unwrap()));
            }
            Type::Int64 => {
                self.subgroup_fields.last_mut().unwrap().push(Arc::new(PrimitiveTypeBuilder::new(&flattened_column_name, parquet::basic::Type::INT64).with_id(self.current_id).build().unwrap()));
            }
            Type::Uint64 => {
                self.subgroup_fields.last_mut().unwrap().push(Arc::new(PrimitiveTypeBuilder::new(&flattened_column_name, parquet::basic::Type::INT64).with_id(self.current_id).with_logical_type(Some(LogicalType::Integer { bit_width: 64, is_signed: false })).build().unwrap()));
            }
            Type::Int32 => {
                self.subgroup_fields.last_mut().unwrap().push(Arc::new(PrimitiveTypeBuilder::new(&flattened_column_name, parquet::basic::Type::INT32).with_id(self.current_id).build().unwrap()));
            }
            Type::Fixed64 => {
                self.subgroup_fields.last_mut().unwrap().push(Arc::new(PrimitiveTypeBuilder::new(&flattened_column_name, parquet::basic::Type::DOUBLE).with_id(self.current_id).build().unwrap()));
            }
            Type::Fixed32 => {
                self.subgroup_fields.last_mut().unwrap().push(Arc::new(PrimitiveTypeBuilder::new(&flattened_column_name, parquet::basic::Type::DOUBLE).with_id(self.current_id).build().unwrap()));
            }
            Type::Bool => {
                self.subgroup_fields.last_mut().unwrap().push(Arc::new(PrimitiveTypeBuilder::new(&flattened_column_name, parquet::basic::Type::DOUBLE).with_id(self.current_id).build().unwrap()));
            }
            Type::String => {
                self.subgroup_fields.last_mut().unwrap().push(Arc::new(PrimitiveTypeBuilder::new(&flattened_column_name, parquet::basic::Type::DOUBLE).with_id(self.current_id).build().unwrap()));
            }
            Type::Group => {
                self.subgroup_fields.last_mut().unwrap().push(Arc::new(PrimitiveTypeBuilder::new(&flattened_column_name, parquet::basic::Type::DOUBLE).with_id(self.current_id).build().unwrap()));
            }
            Type::Message => {
                self.subgroup_fields.last_mut().unwrap().push(Arc::new(PrimitiveTypeBuilder::new(&flattened_column_name, parquet::basic::Type::DOUBLE).with_id(self.current_id).build().unwrap()));
            }
            Type::Bytes => {
                self.subgroup_fields.last_mut().unwrap().push(Arc::new(PrimitiveTypeBuilder::new(&flattened_column_name, parquet::basic::Type::DOUBLE).with_id(self.current_id).build().unwrap()));
            }
            Type::Uint32 => {
                self.subgroup_fields.last_mut().unwrap().push(Arc::new(PrimitiveTypeBuilder::new(&flattened_column_name, parquet::basic::Type::DOUBLE).with_id(self.current_id).build().unwrap()));
            }
            Type::Enum => {
                self.subgroup_fields.last_mut().unwrap().push(Arc::new(PrimitiveTypeBuilder::new(&flattened_column_name, parquet::basic::Type::DOUBLE).with_id(self.current_id).build().unwrap()));
            }
            Type::Sfixed32 => {
                self.subgroup_fields.last_mut().unwrap().push(Arc::new(PrimitiveTypeBuilder::new(&flattened_column_name, parquet::basic::Type::DOUBLE).with_id(self.current_id).build().unwrap()));
            }
            Type::Sfixed64 => {
                self.subgroup_fields.last_mut().unwrap().push(Arc::new(PrimitiveTypeBuilder::new(&flattened_column_name, parquet::basic::Type::DOUBLE).with_id(self.current_id).build().unwrap()));
            }
            Type::Sint32 => {
                self.subgroup_fields.last_mut().unwrap().push(Arc::new(PrimitiveTypeBuilder::new(&flattened_column_name, parquet::basic::Type::DOUBLE).with_id(self.current_id).build().unwrap()));
            }
            Type::Sint64 => {
                self.subgroup_fields.last_mut().unwrap().push(Arc::new(PrimitiveTypeBuilder::new(&flattened_column_name, parquet::basic::Type::DOUBLE).with_id(self.current_id).build().unwrap()));
            }
        }

        self.current_id += 1;
    }
}