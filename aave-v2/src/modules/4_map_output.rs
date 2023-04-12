use crate::pb::aave_v2::v1::{ATokenBalances, ATokenSupplies, AaveV2Events, Output};

#[substreams::handlers::map]
fn map_output(
    events: AaveV2Events,
    balances: ATokenBalances,
    supply: ATokenSupplies,
) -> Result<Output, substreams::errors::Error> {
    Ok(Output {
        events: Some(events),
        balances: Some(balances),
        supplies: Some(supply),
    })
}
