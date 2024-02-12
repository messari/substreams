CREATE TABLE IF NOT EXISTS factory_pair_created (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "pair" VARCHAR(40),
    "param3" DECIMAL,
    "token0" VARCHAR(40),
    "token1" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);

CREATE TABLE IF NOT EXISTS pools_approval (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "evt_address" VARCHAR(40),
    "owner" VARCHAR(40),
    "spender" VARCHAR(40),
    "value" DECIMAL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS pools_burn (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "evt_address" VARCHAR(40),
    "amount0" DECIMAL,
    "amount1" DECIMAL,
    "sender" VARCHAR(40),
    "to" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS pools_mint (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "evt_address" VARCHAR(40),
    "amount0" DECIMAL,
    "amount1" DECIMAL,
    "sender" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS pools_swap (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "evt_address" VARCHAR(40),
    "amount0_in" DECIMAL,
    "amount0_out" DECIMAL,
    "amount1_in" DECIMAL,
    "amount1_out" DECIMAL,
    "sender" VARCHAR(40),
    "to" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS pools_sync (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "evt_address" VARCHAR(40),
    "reserve0" DECIMAL,
    "reserve1" DECIMAL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS pools_transfer (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "evt_address" VARCHAR(40),
    "from" VARCHAR(40),
    "to" VARCHAR(40),
    "value" DECIMAL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
