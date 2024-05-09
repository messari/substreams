use substreams::store::StoreNew;
use substreams::store::StoreSetIfNotExists;
use substreams::store::StoreSetIfNotExistsString;

use crate::pb::contract::v1::Factories;

#[substreams::handlers::store]
pub fn store_factories(factories_added: Factories, store: StoreSetIfNotExistsString) {
    for factory in factories_added.factories {
        store.set_if_not_exists(0, &factory.address, &"Added".to_string());
    }
}
