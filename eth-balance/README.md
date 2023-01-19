# ETH Balance Substream

The purpose of this substream is to get a all ETH transferred in every transaction and the following balance changes for various reasons.

## Data Structure

Every transaction on ethereum is made a `Transfer` in the `evm_token` proto. A transfer without any "value" will still be available. There should still be eth balance changes from gas at least which we will want to see in the data.

A snippet of the important proto definitions looks like this:

```protobuf
message Transfer {
  string tx_hash = 1;
  uint64 block_number = 2;
  uint64 timestamp = 3;
  uint32 log_index = 4;
  Token token = 5;
  string to = 6;
  string from = 7;
  string amount = 8; // BigInt, in token's native amount
  optional string amount_usd = 9;
  repeated TokenBalance balance_changes = 10;
}

// balance changes
message TokenBalance {
  uint64 log_ordinal = 1;
  Token token = 2;
  string address = 3; // account address of the balance change
  string old_balance = 4; // BigInt, in token's native amount
  string new_balance = 5; // BigInt, in token's native amount
  optional int32 reason = 6;
}
```

## Notes

- The only module in this substream is `map_balances` and it takes in eth blocks and maps the balance changes.
- The balance changes have a reason associated with them which provides more context to the data.
- Since we use a map module there is no "back-processing", you can run from any block. See the next section on how to run this on a section of code.

## Running

To specify the start and end block that you want to see balances for change the run command in the `Makefile` like this:

```makefile
substreams run -e mainnet.eth.streamingfast.io:443 substreams.yaml map_balances -s start-block -t end-block
```

Then run the command:

```bash
make run
```
