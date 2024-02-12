CREATE TABLE IF NOT EXISTS factory_pair_created (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "pair" VARCHAR(40),
    "param3" UInt256,
    "token0" VARCHAR(40),
    "token1" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS pools_approval (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "evt_address" VARCHAR(40),
    "owner" VARCHAR(40),
    "spender" VARCHAR(40),
    "value" UInt256
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS pools_burn (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "evt_address" VARCHAR(40),
    "amount0" UInt256,
    "amount1" UInt256,
    "sender" VARCHAR(40),
    "to" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS pools_mint (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "evt_address" VARCHAR(40),
    "amount0" UInt256,
    "amount1" UInt256,
    "sender" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS pools_swap (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "evt_address" VARCHAR(40),
    "amount0_in" UInt256,
    "amount0_out" UInt256,
    "amount1_in" UInt256,
    "amount1_out" UInt256,
    "sender" VARCHAR(40),
    "to" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS pools_sync (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "evt_address" VARCHAR(40),
    "reserve0" UInt128,
    "reserve1" UInt128
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS pools_transfer (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "evt_address" VARCHAR(40),
    "from" VARCHAR(40),
    "to" VARCHAR(40),
    "value" UInt256
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");


