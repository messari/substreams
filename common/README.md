# Common Definitions

This folder contains common definitions for protobufs used more than once. We intentionally have this in order to standardize the way we define the data to make it easier for the end user to "plug-n-play".

We do NOT want to "over-standardize" on a substream level. Substreams works with raw, lower-level data. We do not want to lose any descriptiveness at this layer, therefore it is important to not generalize any data to try and fit a standard.

## Common Proto

TODO

## DEX AMM Proto

TODO

##  EVM Token Proto

This proto is designed to support fungible tokens on evm chains. In order to preserve the data from each chain we have separate definitions for tokens across chain implementations.
