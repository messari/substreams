#[path = "1_map_domain.rs"]
mod map_domain;

#[path = "2_store_ens_record.rs"]
mod store_ens_record;

#[path = "3_store_registrant_ens.rs"]
mod store_registrant_ens;

#[path = "4_store_registrant_address.rs"]
mod store_registrant_address;

pub use map_domain::map_domain;
pub use store_ens_record::store_ens_record;
pub use store_registrant_address::store_registrant_address;
pub use store_registrant_ens::store_registrant_ens;
