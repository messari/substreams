.PHONY: build_all
build_all:
	$(MAKE) -C ../erc20-price build
	$(MAKE) -C ../erc20-price pack
	cargo build --target wasm32-unknown-unknown --release

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: run
run:
	substreams run -e goerli.eth.streamingfast.io:443 substreams.yaml map_pool_created -s 2638438 -t 2667860
.PHONY: pack
pack:
	substreams pack ./substreams.yaml
