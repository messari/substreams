# NOTE: The erc20-price-substreams-v0.1.0.spkg must be available at the root of this crate.

.PHONY: codegen
codegen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/ethereum,sf/substreams,google"

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: example
example:
	substreams run -e api-dev.streamingfast.io:443 substreams.yaml map_market_cap -s 13000000 -t +1
