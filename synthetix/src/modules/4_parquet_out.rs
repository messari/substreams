use substreams::pb::substreams::Clock;

use crate::pb::synthetix::v1::{
    AccumulatedRewardsPerShare, EscrowRewards, LiquidatorRewards, ParquetOut, ParquetOuts,
    TokenBalances,
};

#[substreams::handlers::map]
fn parquet_out(
    clock: Clock,
    balances: TokenBalances,
    rewards: EscrowRewards,
    liq_rewards: LiquidatorRewards,
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

    for liq in liq_rewards.rewards {
        let out = ParquetOut {
            synthetix: Some(
                crate::pb::synthetix::v1::parquet_out::Synthetix::LiquidatorReward(liq),
            ),
        };
        outs.push(out);
    }

    if let Some(accumulated) = liq_rewards.accumulated_rewards_per_share {
        let acc = AccumulatedRewardsPerShare {
            accumulated_rewards_per_share: Some(accumulated.into()),
            timestamp: Some(clock.into()),
        };
        let out = ParquetOut {
            synthetix: Some(
                crate::pb::synthetix::v1::parquet_out::Synthetix::AccumulatedRewardsPerShare(acc),
            ),
        };
        outs.push(out);
    }

    Ok(ParquetOuts { outs })
}
