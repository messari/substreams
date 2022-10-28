# NOTE: The erc20-price-substreams-v0.1.0.spkg must be available at the root of this crate.

.PHONY: codegen
codegen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/ethereum,sf/substreams,google"

.PHONY: build
build:
	$(MAKE) -C ../erc20-price build
	$(MAKE) -C ../erc20-price pack
	cargo build --target wasm32-unknown-unknown --release

.PHONY: run
run:
	substreams run -e api-dev.streamingfast.io:443 substreams.yaml map_market_cap -s 13000700 -t +1
