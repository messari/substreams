.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: run
run:
	substreams run -e polygon.streamingfast.io:443 substreams.yaml map_output --start-block 12690033 --stop-block +1
