use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::{StoreAdd, StoreSet};
use substreams::{log, store};

pub trait StoreSetter {
    type Input;

    fn add_value<K: AsRef<str>>(&self, _key: K, _value: &Self::Input) {
        log::info!("set_value not implemented")
    }

    fn set_value<K: AsRef<str>>(&self, _key: K, _value: &Self::Input) {
        log::info!("set_value not implemented");
    }
}

impl StoreSetter for store::StoreAddBigInt {
    type Input = BigInt;

    fn add_value<K: AsRef<str>>(&self, key: K, value: &Self::Input) {
        self.add(0, key, value)
    }
}

impl StoreSetter for store::StoreAddBigDecimal {
    type Input = BigDecimal;

    fn add_value<K: AsRef<str>>(&self, key: K, value: &Self::Input) {
        self.add(0, key, value)
    }
}

impl StoreSetter for store::StoreSetBigInt {
    type Input = BigInt;

    fn set_value<K: AsRef<str>>(&self, key: K, value: &Self::Input) {
        self.set(0, key, value)
    }
}

impl StoreSetter for store::StoreSetBigDecimal {
    type Input = BigDecimal;

    fn set_value<K: AsRef<str>>(&self, key: K, value: &Self::Input) {
        self.set(0, key, value)
    }
}
