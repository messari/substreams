.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: run
run:
	substreams run -e mainnet.sol.streamingfast.io:443 substreams.yaml store_test -s 0 -t +10
