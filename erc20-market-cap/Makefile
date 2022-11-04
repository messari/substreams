.PHONY: build
build:
	$(MAKE) -C ../erc20-price build
	$(MAKE) -C ../erc20-price pack
	cargo build --target wasm32-unknown-unknown --release

.PHONY: run
run:
	substreams run -e api-dev.streamingfast.io:443 substreams.yaml map_market_cap -s 13000700 -t +1
