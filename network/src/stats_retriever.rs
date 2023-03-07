use crate::pb::network::v1::Stats;
use crate::store_key::StoreKey;
use crate::store_retriever::StoreRetriever;
use crate::utils::{get_latest_day, get_latest_hour};
use substreams::scalar::{BigInt, BigDecimal};
use substreams::store;
use substreams_entity_change::pb::entity::entity_change::Operation;
use substreams_entity_change::pb::entity::EntityChange;

pub(crate) struct StatsRetriever<'a> {
    aggregation_retriever: StoreRetriever<'a>,
    max_value_retriever: StoreRetriever<'a>,
    min_value_retriever: StoreRetriever<'a>,
    parent_entity_id: String,
}

impl<'a> StatsRetriever<'a> {
    pub(crate) fn new(aggregation_store: &'a store::StoreGetBigInt, max_store: &'a store::StoreGetBigInt, min_store: &'a store::StoreGetBigInt, timestamp: i64) -> Self {
        // If we are taking a day/hour snapshot it will be of the previous day/hour
        let day_timestamp = get_latest_day(timestamp) - 1;
        let hour_timestamp = get_latest_hour(timestamp) - 1;

        StatsRetriever {
            aggregation_retriever: StoreRetriever::new(aggregation_store, Some(day_timestamp), Some(hour_timestamp)),
            max_value_retriever: StoreRetriever::new(max_store, Some(day_timestamp), Some(hour_timestamp)),
            min_value_retriever: StoreRetriever::new(min_store, Some(day_timestamp), Some(hour_timestamp)),
            parent_entity_id: Default::default(), // To be overridden
        }
    }

    pub(crate) fn set_entity_id(&mut self, entity_id: String) {
        self.parent_entity_id = entity_id;
    }

    pub(crate) fn get_aggregation_retriever(&self) -> &StoreRetriever {
        &self.aggregation_retriever
    }

    pub(crate) fn get_id(&self, field_name: &str) -> String {
        format!("{}-{}", self.parent_entity_id, field_name)
    }

    pub(crate) fn get_total_stats(&self, field_name: &str, variable_key: StoreKey, count_key: StoreKey, is_creation_operation: bool) -> EntityChange {
        let count = self.aggregation_retriever.get_total_sum(count_key).to_u64() as i32;
        let count_big_decimal = BigDecimal::from(count);

        let (sum, sum_squares) = self.aggregation_retriever.get_total_stats_values(variable_key.clone());

        let mean = BigDecimal::from(sum.clone()) / count_big_decimal.clone();
        let variance = (BigDecimal::from(sum_squares) / count_big_decimal) - mean.squared();

        let mut stats_entity_change = if is_creation_operation {
            EntityChange::new("Stats", &self.get_id(field_name), 0, Operation::Create)
        } else {
            EntityChange::new("Stats", &self.get_id(field_name), 0, Operation::Update)
        };

        stats_entity_change.change("id", self.get_id(field_name))
            .change("count", count)
            .change("mean", mean)
            .change("max", self.max_value_retriever.get_total_min_or_max_value(variable_key.clone()))
            .change("min", self.min_value_retriever.get_total_min_or_max_value(variable_key))
            .change("sum", sum)
            .change("variance", variance);

        stats_entity_change
    }

    pub(crate) fn get_day_stats(&self, field_name: &str, variable_key: StoreKey, count_key: StoreKey) -> EntityChange {
        let count = self.aggregation_retriever.get_day_sum(count_key).to_u64() as i32;
        let count_big_decimal = BigDecimal::from(count);

        let (sum, sum_squares) = self.aggregation_retriever.get_day_stats_values(variable_key.clone());

        let mean = BigDecimal::from(sum.clone()) / count_big_decimal.clone();
        let variance = (BigDecimal::from(sum_squares) / count_big_decimal) - mean.squared();

        let mut stats_entity_change = EntityChange::new("Stats", &self.get_id(field_name), 0, Operation::Create);

        stats_entity_change.change("id", self.get_id(field_name))
            .change("count", count)
            .change("mean", mean)
            .change("max", self.max_value_retriever.get_day_min_or_max_value(variable_key.clone()))
            .change("min", self.min_value_retriever.get_day_min_or_max_value(variable_key))
            .change("sum", sum)
            .change("variance", variance);

        stats_entity_change
    }

    pub(crate) fn get_hour_stats_entity_change(&self, field_name: &str, variable_key: StoreKey, count_key: StoreKey) -> EntityChange {
        let count = self.aggregation_retriever.get_hour_sum(count_key).to_u64() as i32;
        let count_big_decimal = BigDecimal::from(count);

        let (sum, sum_squares) = self.aggregation_retriever.get_hour_stats_values(variable_key.clone());

        let mean = BigDecimal::from(sum.clone()) / count_big_decimal.clone();
        let variance = (BigDecimal::from(sum_squares) / count_big_decimal) - mean.squared();

        let mut stats_entity_change = EntityChange::new("Stats", &self.get_id(field_name), 0, Operation::Create);

        stats_entity_change.change("id", self.get_id(field_name))
            .change("count", count)
            .change("mean", mean)
            .change("max", self.max_value_retriever.get_hour_min_or_max_value(variable_key.clone()))
            .change("min", self.min_value_retriever.get_hour_min_or_max_value(variable_key))
            .change("sum", sum)
            .change("variance", variance);

        stats_entity_change
    }

    pub(crate) fn get_zero_stats_entity_change(&self, field_name: &str) -> EntityChange {
        let mut stats_entity_change = EntityChange::new("Stats", &self.get_id(field_name), 0, Operation::Create);

        stats_entity_change.change("id", self.get_id(field_name))
            .change("count", 0)
            .change("mean", BigDecimal::zero())
            .change("max", BigInt::zero())
            .change("min", BigInt::zero())
            .change("sum", BigInt::zero())
            .change("variance", BigDecimal::zero());

        stats_entity_change
    }
}

trait Squared {
    fn squared(&self) -> Self;
}

impl Squared for BigDecimal {
    fn squared(&self) -> Self {
        self.clone() * self.clone()
    }
}