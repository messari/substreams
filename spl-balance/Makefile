.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: run
run:
	substreams run -e mainnet.sol.streamingfast.io:443 substreams.yaml map_balances -s 173531340 -t +1
