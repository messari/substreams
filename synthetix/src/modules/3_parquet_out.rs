use crate::pb::synthetix::v1::{EscrowRewards, ParquetOut, ParquetOuts, TokenBalances};

#[substreams::handlers::map]
fn parquet_out(
    balances: TokenBalances,
    rewards: EscrowRewards,
) -> Result<ParquetOuts, substreams::errors::Error> {
    let mut outs = vec![];

    for balance in balances.balances {
        let out = ParquetOut {
            synthetix: Some(crate::pb::synthetix::v1::parquet_out::Synthetix::Balance(
                balance,
            )),
        };
        outs.push(out);
    }

    for reward in rewards.rewards {
        let out = ParquetOut {
            synthetix: Some(crate::pb::synthetix::v1::parquet_out::Synthetix::EscrowReward(reward)),
        };
        outs.push(out);
    }

    Ok(ParquetOuts { outs })
}
