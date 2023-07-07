use serde_json::json;
use substreams::store::{DeltaProto, Deltas};

use crate::pb::ens::v1 as ENS;

#[substreams::handlers::map]
pub fn ens_jsonl_out(
    ens_record_deltas: Deltas<DeltaProto<ENS::Domain>>,
) -> Result<ENS::Lines, substreams::errors::Error> {
    let mut lines = vec![];

    for delta in ens_record_deltas.deltas {
        if !delta.key.as_str().ends_with(".eth") {
            continue;
        }

        lines = vec![json!({
            "ens_name": delta.new_value.ens_name,
            "name_hash": delta.new_value.name_hash,
            "label_name": delta.new_value.label_name,
            "label_hash": delta.new_value.label_hash,
            "controller_address": delta.new_value.controller_address,
            "registrant_address": delta.new_value.registrant_address,
            "transaction_hash": delta.new_value.transaction_hash,
            "block_number": delta.new_value.block_number,
        })
        .to_string()];
    }

    Ok(ENS::Lines { lines })
}
