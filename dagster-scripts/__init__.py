import os

from dagster import (
    Definitions,
    get_dagster_logger,
    multiprocess_executor,
)

from resources import (
    RESOURCES_PROD,
    RESOURCES_STAGING,
    RESOURCES_DEV,
)

from substreams_processor import substreams_processing_scheduler, substreams_processor
from snowflake_uploader import upload_new_parquet_files_to_snowflake

logger = get_dagster_logger()

# ***** RESOURCES *****
resource_defs_by_deployment_name = {
    "prod": RESOURCES_PROD,
    "stage": RESOURCES_STAGING,
    "dev": RESOURCES_DEV,
}

# ***** DEPLOYMENTS *****
deployment_name = os.environ.get("ENVIRONMENT", "dev")
logger.info(f"Using deployment {deployment_name}")
resource_defs = resource_defs_by_deployment_name[deployment_name]

defs = Definitions(
    jobs=[substreams_processor],
    resources=resource_defs,
    schedules = [substreams_processing_scheduler, upload_new_parquet_files_to_snowflake],
    executor=multiprocess_executor,
)