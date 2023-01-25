.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: run
run:
	substreams run substreams.yaml map_transfers -s 1000 -t +5 --debug-modules-initial-snapshot=store_pools