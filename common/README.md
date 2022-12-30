# Common Definitions

This folder contains common definitions for protobufs used more than once. We intentionally have this in order to standardize the way we define the data to make it easier for the end user to "plug-n-play".

## Common Proto

TODO

## DEX AMM Proto

TODO

## Token Proto

The token proto is designed to store the token balances of all accounts throughout the blockchains history. It is designed to be network-agnostic and support all tokens. Token's can be blockchain native, erc20s, or any other derivative.

### Definitions

The first part are the token definitions. This is simple, and defines the tokens that are supported in the substream implementation.

> Note: if the token is a native token (e.g. ethereum), then the address is the `0x0` address and the other values need to be hardcoded.
```protobuf
message Tokens {
  repeated Token items = 1;
}

message Token {
  string address = 1;
  string name = 2;
  string symbol = 3;
  uint64 decimals = 4;
}
```

The transfers track where the tokens move to/from. In most cases we can fill in every field. A `log_index` or `log_ordinal` may not always be possible. In addition a mint or burn will have either the `to` or `from` address as the `0x0` address.

```protobuf
message Transfers {
  repeated Transfer items = 1;
}

message Transfer {
  string tx_hash = 1;
  optional uint32 log_index = 2;
  optional uint64 log_ordinal = 3;
  Token token = 4;
  string from = 5;
  string to = 6;
  string amount = 7; // BigInt, in token's native amount
  optional string reason = 8;
}
```

`TokenBalance` is simple, and defines the balance of a given token in an account.
```protobuf
message TokenBalance {
  string token_address = 1;
  string balance = 2; // BigInt, in token's native amount
}

message Account {
  string address = 1;
  repeated TokenBalance balances = 2;
}
```

### How to Use

TODO
