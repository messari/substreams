.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: run
run:
	substreams run -e api-dev.streamingfast.io:443 substreams.yaml map_eth_price -s 13000000 -t +10

.PHONY: pack
pack:
	substreams pack ./substreams.yaml --output-file {manifestDir}/target/{spkgDefaultName}