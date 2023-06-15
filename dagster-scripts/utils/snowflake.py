from dagster_snowflake.resources import SnowflakeConnection
from dagster_snowflake import SnowflakeResource
from dagster import InputContext, OutputContext
from typing import Union
import os

def get_snowflake_connection(context: Union[InputContext, OutputContext]):
    # User + pass may be passed as raw strings or as {"env", "<var>"}
    user = context.resource_config.get("user")
    if isinstance(user, dict):
        user = user["env"]
        user = os.getenv(user)

    password = context.resource_config.get("password")
    if isinstance(password, dict):
        password = password["env"]
        password = os.getenv(password)

    snowflake_connection_params = dict(
        connector="sqlalchemy",
        user=user,
        password=password,
        account=context.resource_config.get("account"),
        database=context.resource_config.get("database"),
    )

    SnowflakeConnection(
        config=snowflake_connection_params,
        log=context.log,
        snowflake_connection_resource=SnowflakeResource(**snowflake_connection_params),
    ).get_connection(raw_conn=False)