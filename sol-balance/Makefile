.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: run
run:
	substreams run -e mainnet.sol.streamingfast.io:443 substreams.yaml store_balance -s 10000 -t +10001
