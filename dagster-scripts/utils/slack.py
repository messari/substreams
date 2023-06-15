from dagster import InputContext, OutputContext
from dagster import op, success_hook, failure_hook
from typing import Optional, Union
import time


SUBSTREAMS_DWH_NOTIFICATIONS_SLACK_CHANNEL = "substreams_dwh_notifications"
MAX_PROCESSING_TIME = 60*60*24 # One day

def get_latest_slack_message_for_spkg(context: Union[InputContext, OutputContext], spkg_name: str) -> Optional[str]:
	pagination_number = 0
	latest_timestamp = time.now()
	earliest_timestamp = latest_timestamp - MAX_PROCESSING_TIME # (I will make a cutoff for how long the process cmd can run for to avoid hanging issues etc)
	messages = context.resources.slack.get_conversation_history(SUBSTREAMS_DWH_NOTIFICATIONS_SLACK_CHANNEL, earliest_timestamp, latest_timestamp, pagination_number)
	pagination_number += messages.len
	while messages is not None:
		for message in messages:
			if message.text.contains(spkg_name):
				return message
		messages = context.resources.slack.get_conversation_history(SUBSTREAMS_DWH_NOTIFICATIONS_SLACK_CHANNEL, earliest_timestamp, latest_timestamp, pagination_number)
		pagination_number += messages.len
	return None

@success_hook(required_resource_keys={"slack"})
def slack_success(context: Union[InputContext, OutputContext], message: str):
    context.resources.slack.chat_postMessage(channel=SUBSTREAMS_DWH_NOTIFICATIONS_SLACK_CHANNEL, text=message)
    
@failure_hook(required_resource_keys={"slack"})
def slack_success(context: Union[InputContext, OutputContext], message: str):
    context.resources.slack.chat_postMessage(channel=SUBSTREAMS_DWH_NOTIFICATIONS_SLACK_CHANNEL, text=message)