.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: run
run:
	substreams run -e mainnet.sol.streamingfast.io:443 substreams.yaml map_balances -s 174257750 -t +1
