# SOL Balance Substream

This substream is designed to store the SOL balance of every account.

### Notes  

- We are not able fully map all the accounts to SOL balance changes.
  - There is a missing field [`address_table_lookup`](https://github.com/streamingfast/firehose-solana/blob/develop/proto/sf/solana/type/v1/type.proto#L38) that stores the rest of the addresses.
  - See the issue filed [here](https://github.com/streamingfast/substreams/issues/144) to track the status.
