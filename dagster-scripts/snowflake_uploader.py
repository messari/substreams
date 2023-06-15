from utils.io_utils import file_awaiting_upload_to_snowflake, set_data_file_as_uploaded_to_snowflake
from utils.snowflake import get_snowflake_connection
from dagster import InputContext, OutputContext
from dagster_aws.s3.sensor import get_s3_keys
from dagster import schedule, SkipReason, op
from slack import slack_op
from typing import Union

def get_table_info(file_key) -> str:
    print("TODO: What is the standard table design that you tend to use here??")

@op(required_resource_keys={'substreams_data_bucket'}) # TODO: Add some retry logic for this job
def process_newly_uploaded_substream_data(context: Union[InputContext, OutputContext], new_parquet_files: list[str]):
    # Group all the files by table
    table_to_file_mappings = {}
    for parquet_file in new_parquet_files:
        table_info = get_table_info(parquet_file)
        if table_info in table_to_file_mappings:
            table_to_file_mappings[table_info].append(parquet_file)
        else:
            table_to_file_mappings[table_info] = [parquet_file]
    
    snowflake_connection = get_snowflake_connection(context)

    for table_info, parquet_files in table_to_file_mappings.items():
        files_statement = f"['s3://{context.resources.substreams_data_bucket}/{parquet_files[0]}'"
        for parquet_file in parquet_files[1:]:
            files_statement += f", 's3://{context.resources.substreams_data_bucket}/{parquet_file}'"
        files_statement += "]"

        snowflake_connection.cursor().execute(f"""
            CREATE TABLE IF NOT EXISTS {table_info}
            USING TEMPLATE (
                SELECT ARRAY_AGG(OBJECT_CONSTRUCT(*))
                FROM TABLE(
                    INFER_SCHEMA(
                    LOCATION=>'{parquet_files[0]}',
                    FILE_FORMAT=>parquet
                    )
                ))
            CLUSTER BY block_number;
                             
            COPY INTO {table_info} FROM {files_statement}
                FILE_FORMAT=parquet;
            """)
        for parquet_file in parquet_files: # TODO: On error for below line we need to push an error to slack with all the files that have yet to change metadata to "uploaded_to_snowflake"
            set_data_file_as_uploaded_to_snowflake(context.resources.substreams_data_bucket, parquet_file)

@schedule(cron_schedule="*/5 * * * *", required_resource_keys={'substreams_data_bucket'}) # runs every 5 minutes
def upload_new_parquet_files_to_snowflake(context: Union[InputContext, OutputContext]):
    since_key = context.cursor or None
    new_s3_keys = get_s3_keys(context.resources.substreams_data_bucket, "substreams", since_key=since_key)
    if not new_s3_keys:
        return SkipReason("No new substream data has been uploaded to s3 since job was last run.")
    last_key = new_s3_keys[-1]
    context.update_cursor(last_key)

    new_file_keys = [for s3_key in new_s3_keys if file_awaiting_upload_to_snowflake(context.resources.substreams_data_bucket, s3_key)]
    process_newly_uploaded_substream_data(new_file_keys)
    return None