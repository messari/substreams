.PHONY: codegen
codegen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/solana,sf/substreams,google"

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: run
run:
	substreams run -e mainnet.sol.streamingfast.io:443 substreams.yaml store_balance -s 1000 -t +10
