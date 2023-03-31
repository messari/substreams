.PHONY: build run all

build:
	cargo build --target wasm32-unknown-unknown --release

run:
	substreams run -e mainnet.eth.streamingfast.io:443 substreams.yaml map_entity_changes -s 5350000 -t 5350001

all: build run
