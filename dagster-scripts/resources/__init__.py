from dagster_slack import slack_resource
import os

RESOURCES_PROD = {
    "slack": slack_resource,
    "substreams_data_bucket": "data-warehouse-load-427049689281-dev"
}

RESOURCES_STAGING = {
    "slack": slack_resource,
    "substreams_data_bucket": "data-warehouse-load-427049689281-stage"
}

RESOURCES_DEV = {
    "slack": slack_resource,
    "substreams_data_bucket": "data-warehouse-load-427049689281-prod"
}