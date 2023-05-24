use substreams_entity_change::pb::entity::EntityChanges;

use substreams_helper::convert::BigIntDeserializeExt;
use substreams_helper::tables::Tables;

use crate::pb::synthetix::v1::{
    BalanceType, EscrowContractVersion, EscrowReward, EscrowRewards, TokenBalance, TokenBalances,
};

#[substreams::handlers::map]
fn graph_out(
    balances: TokenBalances,
    rewards: EscrowRewards,
) -> Result<EntityChanges, substreams::errors::Error> {
    let mut tables = Tables::new();

    for balance in balances.balances {
        let ids = token_balance_ids(&balance);
        let pbts = balance.timestamp.unwrap();
        let block_num = pbts.block_number;
        let timestamp = pbts.timestamp;
        let amount = balance.balance.unwrap().deserialize();
        tables
            .update_row("TokenBalance", ids.0.clone())
            .set("token", balance.token)
            .set("holder", balance.holder)
            .set_bigint("balance", amount.as_ref())
            .set_bigint("timestamp", &timestamp.into())
            .set_bigint("block", &block_num.into());

        tables
            .create_row("TokenBalanceSnapshot", ids.1)
            .set("tokenBalance", ids.0)
            .set_bigint("balance", amount.as_ref())
            .set_bigint("timestamp", &timestamp.into())
            .set_bigint("block", &block_num.into());
    }

    for reward in rewards.rewards {
        let ids = reward_ids(&reward);
        let pbts = reward.timestamp.unwrap();
        let block_num = pbts.block_number;
        let timestamp = pbts.timestamp;
        let amount = reward.balance.unwrap().deserialize();
        tables
            .update_row("EscrowReward", ids.0.clone())
            .set(
                "balance_type",
                escrow_balance_type_to_graphql_enum(
                    BalanceType::from_i32(reward.balance_type).unwrap(),
                ),
            )
            .set(
                "contract_version",
                escrow_contract_version_to_graphql_enum(
                    EscrowContractVersion::from_i32(reward.escrow_contract_version).unwrap(),
                ),
            )
            .set("holder", reward.holder)
            .set_bigint("balance", amount.as_ref())
            .set_bigint("timestamp", &timestamp.into())
            .set_bigint("block", &block_num.into());

        tables
            .create_row("EscrowRewardSnapshot", ids.1)
            .set("escrowReward", ids.0)
            .set_bigint("balance", amount.as_ref())
            .set_bigint("timestamp", &timestamp.into())
            .set_bigint("block", &block_num.into());
    }

    Ok(tables.to_entity_changes())
}

fn token_balance_ids(balance: &TokenBalance) -> (String, String) {
    let timestamp = balance.timestamp.as_ref().unwrap();
    let id = format!("{}-{}", balance.token, balance.holder);
    return (id.clone(), format!("{}-{}", id, timestamp.block_number));
}

fn reward_ids(reward: &EscrowReward) -> (String, String) {
    let timestamp = reward.timestamp.as_ref().unwrap();
    let id = format!(
        "{}-{}-{}",
        reward.balance_type, reward.escrow_contract_version, reward.holder
    );
    return (id.clone(), format!("{}-{}", id, timestamp.block_number));
}

fn escrow_balance_type_to_graphql_enum(btype: BalanceType) -> String {
    match btype {
        BalanceType::Escrowed => "ESCROWED".to_string(),
        BalanceType::Vested => "VESTED".to_string(),
    }
}

fn escrow_contract_version_to_graphql_enum(version: EscrowContractVersion) -> String {
    match version {
        EscrowContractVersion::V1 => "V1".to_string(),
        EscrowContractVersion::V2 => "V2".to_string(),
    }
}
