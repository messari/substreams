.PHONY: codegen
codegen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/ethereum,sf/substreams,google"

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: example
example:
	substreams run -e api-dev.streamingfast.io:443 substreams.yaml map_price -s 13000000 -t +10
