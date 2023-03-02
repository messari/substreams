use crate::pb::network::v1::Stats;
use crate::store_key::StoreKey;
use crate::store_retriever::StoreRetriever;
use crate::utils::{get_latest_day, get_latest_hour};
use substreams::scalar::{BigInt, BigDecimal};
use substreams::store;

pub(crate) struct StatsRetriever<'a> {
    aggregation_retriever: StoreRetriever<'a>,
    max_value_retriever: StoreRetriever<'a>,
    min_value_retriever: StoreRetriever<'a>,
    entity_id: String,
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
            entity_id: Default::default(), // To be overridden
        }
    }

    pub(crate) fn set_entity_id(&mut self, entity_id: String) {
        self.entity_id = entity_id;
    }

    pub(crate) fn get_aggregation_retriever(&self) -> &StoreRetriever {
        &self.aggregation_retriever
    }

    pub(crate) fn get_total_stats(&self, variable_key: StoreKey, count_key: StoreKey) -> Stats {
        let count = self.aggregation_retriever.get_total_sum(count_key).to_u64() as i32;
        let count_big_decimal = BigDecimal::from(count);

        let (sum, sum_squares) = self.aggregation_retriever.get_total_stats_values(variable_key.clone());

        let mean = BigDecimal::from(sum.clone()) / count_big_decimal.clone();
        let variance = (BigDecimal::from(sum_squares) / count_big_decimal) - mean.squared();

        Stats {
            id: self.entity_id.clone(),
            count,
            mean: Some(mean.into()),
            max: Some(self.max_value_retriever.get_total_min_or_max_value(variable_key.clone()).into()),
            min: Some(self.min_value_retriever.get_total_min_or_max_value(variable_key).into()),
            sum: Some(sum.into()),
            variance: Some(variance.into()),
        }
    }

    pub(crate) fn get_day_stats(&self, variable_key: StoreKey, count_key: StoreKey) -> Stats {
        let count = self.aggregation_retriever.get_day_sum(count_key).to_u64() as i32;
        let count_big_decimal = BigDecimal::from(count);

        let (sum, sum_squares) = self.aggregation_retriever.get_day_stats_values(variable_key.clone());

        let mean = BigDecimal::from(sum.clone()) / count_big_decimal.clone();
        let variance = (BigDecimal::from(sum_squares) / count_big_decimal) - mean.squared();

        Stats {
            id: self.entity_id.clone(),
            count,
            mean: Some(mean.into()),
            max: Some(self.max_value_retriever.get_day_min_or_max_value(variable_key.clone()).into()),
            min: Some(self.min_value_retriever.get_day_min_or_max_value(variable_key).into()),
            sum: Some(sum.into()),
            variance: Some(variance.into()),
        }
    }

    pub(crate) fn get_hour_stats(&self, variable_key: StoreKey, count_key: StoreKey) -> Stats {
        let count = self.aggregation_retriever.get_hour_sum(count_key).to_u64() as i32;
        let count_big_decimal = BigDecimal::from(count);

        let (sum, sum_squares) = self.aggregation_retriever.get_hour_stats_values(variable_key.clone());

        let mean = BigDecimal::from(sum.clone()) / count_big_decimal.clone();
        let variance = (BigDecimal::from(sum_squares) / count_big_decimal) - mean.squared();

        Stats {
            id: self.entity_id.clone(),
            count,
            mean: Some(mean.into()),
            max: Some(self.max_value_retriever.get_hour_min_or_max_value(variable_key.clone()).into()),
            min: Some(self.min_value_retriever.get_hour_min_or_max_value(variable_key).into()),
            sum: Some(sum.into()),
            variance: Some(variance.into()),
        }
    }

    pub(crate) fn get_zero_stats(&self) -> Stats {
        Stats {
            id: self.entity_id.clone(),
            count: 0,
            mean: Some(BigDecimal::zero().into()),
            max: Some(BigInt::zero().into()),
            min: Some(BigInt::zero().into()),
            sum: Some(BigInt::zero().into()),
            variance: Some(BigDecimal::zero().into()),
        }
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