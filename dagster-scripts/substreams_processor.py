from utils.slack import get_latest_slack_message_for_spkg, slack_success, slack_failure
from utils.io_utils import download_spkg_to_local, run_cli_cmd, SUBSTREAMS_BUCKET_NAME
from dagster import schedule, RunRequest, Config, job, op, Definitions
from dagster import InputContext, OutputContext
from typing import Union
import boto3
import json
import time

THRESHOLD_NUM_OF_BLOCKS_TO_PROCESS = 10
MIN_TIME_BETWEEN_PROCESSING_SLACK_ERRORS = 60*30 # 30 minutes
MIN_BLOCK_SIZE_THRESHOLD_FOR_EXTENDED_PROCESSING_NOTIFICATIONS = 10_000

@op
def process_spkg(spkg_local_filepath):
    run_cli_cmd(f"messari process {spkg_local_filepath}")

class SpkgFileConfig(Config):
    spkg_file: str

@job(required_resource_keys={"slack"}) # TODO: Add some retry logic for this job
def substreams_processor(context: Union[InputContext, OutputContext], config: SpkgFileConfig):
    spkg_local_filepath = download_spkg_to_local(config.spkg_file)

    block_range_info_response = run_cli_cmd(f"messari block-range-info {spkg_local_filepath}")
    block_range_info = json.loads(block_range_info_response)

    if "start_block" in block_range_info:
        start_block = int(block_range_info["start_block"])
    else:
        raise Exception("Couldn't get start_block from messari block-range-info response!")
    
    if "stop_block" in block_range_info:
        stop_block = int(block_range_info["stop_block"])
    else:
        raise Exception("Couldn't get stop_block from messari block-range-info response!")
    
    block_range_size = stop_block - start_block
    if block_range_size == 0:
        return

    latest_slack_message_for_spkg = get_latest_slack_message_for_spkg(config.spkg_file)    
    if latest_slack_message_for_spkg.text.contains("error") and time.now()-latest_slack_message_for_spkg.creation_time < MIN_TIME_BETWEEN_PROCESSING_SLACK_ERRORS:
        if block_range_size > MIN_BLOCK_SIZE_THRESHOLD_FOR_EXTENDED_PROCESSING_NOTIFICATIONS:
            hooks = {slack_success(f"{config.spkg_file}: Successfully finished extended processing from block: {start_block} to head of chain: {stop_block}")}
        else:
            hooks = {slack_success(f"{config.spkg_file}: Successfully finished processing from block: {start_block} to head of chain: {stop_block}")}
    else:
        if block_range_size > MIN_BLOCK_SIZE_THRESHOLD_FOR_EXTENDED_PROCESSING_NOTIFICATIONS:
            context.resources.slack.send_message_to_channel(f"{config.spkg_file}: Started extended processing from block: {start_block} to head of chain: {stop_block}")
            hooks = {slack_success(f"{config.spkg_file}: Successfully finished extended processing from block: {start_block} to head of chain: {stop_block}"), 
                     slack_failure(f"{config.spkg_file}: An error occurred when extended processing from block: {start_block} to head of chain: {stop_block}")}
        else:
            hooks = {slack_failure(f"{config.spkg_file}: An error occurred when processing from block: {start_block} to head of chain: {stop_block}")}

    process_spkg(spkg_local_filepath).with_hooks(hooks)

@schedule(cron_schedule="*/5 * * * *", job=substreams_processor) # runs every 5 minutes
def substreams_processing_scheduler():
    spjkg_search_objects = client = boto3.client('s3').list_objects_v2(
        Bucket = "spkg_bucket",
        Prefix = "./spkgs/"
    )

    spkg_files = [search_obj.key for search_obj in spjkg_search_objects if search_obj.key.endswith('.parquet')]

    return [RunRequest(run_key=spkg_file, tags={"one_job": spkg_file}) for spkg_file in spkg_files]


