.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: run
run:
	substreams run -e api-dev.streamingfast.io:443 substreams.yaml unique_tracking -s 0 -t +50
