# SOL Balance Substream

This substream is designed to store the SOL balance of every account.

### Build & Run

```bash
make codegen
make run
```

### Notes

- Currently [`transaction.loaded_writable_addresses`](https://github.com/streamingfast/firehose-solana/blob/develop/proto/sf/solana/type/v1/type.proto#L65) is an unknown field on firesol blocks.
- 
