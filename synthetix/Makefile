.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: run
run:
	substreams run -e mainnet.eth.streamingfast.io:443 substreams.yaml graph_out -s 14784804 --stop-block +1
