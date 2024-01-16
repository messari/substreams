CREATE TABLE IF NOT EXISTS pool_registrations (
    "tx_hash" VARCHAR(64),
    "log_index" INT,
    "block_time" TIMESTAMP,
    "block_number" DECIMAL,
    "pool_id" VARCHAR(64),
    "pool_address" VARCHAR(64),
    "specialization" INT,
    "from_address" VARCHAR(64),
    "to_address" VARCHAR(64),
    PRIMARY KEY(tx_hash, log_index)
);