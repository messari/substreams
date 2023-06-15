import subprocess
import requests
import boto3
import os

MESSARI_CLI_PATH = "/tmp/messari_cli"
SUBSTREAMS_BUCKET_NAME = "spkg_bucket"

SUBSTREAMS_AUTH_URI = 'https://auth.streamingfast.io/v1/auth/issue'
SUBSTREAMS_API_KEY  = os.getenv('SUBSTREAMS_API_KEY')

def auth_substream() -> None:
    r = requests.post(SUBSTREAMS_AUTH_URI, data=f'{{"api_key":"{SUBSTREAMS_API_KEY}"}}')
    os.environ['SUBSTREAMS_API_TOKEN'] = r.json().get('token', '')

def download_file_from_aws(bucket: str, key: str, local_path):
    boto3.resource('s3').download_file(bucket, key, local_path)

def run_cli_cmd(command: str) -> str:
    if os.path.exists(MESSARI_CLI_PATH):
        return
    else:
        download_file_from_aws(SUBSTREAMS_BUCKET_NAME, "cli/messari_cli", MESSARI_CLI_PATH)

    if command.startswith("messari process"):
        auth_substream()

    result = subprocess.run(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE, universal_newlines=True)
    if result.returncode == 0:
        return result.stdout
    else:
        raise Exception(f"Issue when running messari CLI cmd! Error: {result.stderr}")

def download_spkg_to_local(spkg_key: str) -> str:
    spkg_local_filepath = f"/tmp/{spkg_key}"
    download_file_from_aws(SUBSTREAMS_BUCKET_NAME, spkg_key, spkg_local_filepath)
    return spkg_local_filepath

def file_awaiting_upload_to_snowflake(bucket, file_key) -> bool:
    metadata = boto3.resource('s3').head_object(Bucket=bucket, Key=file_key)
    if "substream_data" in metadata:
        return metadata["substream_data"] == "AWAITING_UPLOAD_TO_SNOWFLAKE"
    else:
        return False

def set_data_file_as_uploaded_to_snowflake(bucket, filekey):
    s3_object = boto3.resource('s3').Object(bucket, filekey)
    s3_object.metadata.update({'substreams_data':'UPLOADED_TO_SNOWFLAKE'})
    s3_object.copy_from(CopySource={'Bucket': bucket, 'Key': filekey}, Metadata=s3_object.metadata, MetadataDirective='REPLACE')