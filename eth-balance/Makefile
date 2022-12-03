.PHONY: codegen
codegen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/ethereum,sf/substreams,google"

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: run
run:
	substreams run -e mainnet.eth.streamingfast.io:443 substreams.yaml store_balance -s 14690152
