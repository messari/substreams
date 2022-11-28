.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: run
run:
	substreams run -e mainnet.eth.streamingfast.io:443 substreams.yaml store_chainlink_price -s 10606500 -t +1000000

.PHONY: pack
pack:
	substreams pack ./substreams.yaml --output-file target/erc20-price-substreams-v0.1.0.spkg