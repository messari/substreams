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
	substreams run -e mainnet.eth.streamingfast.io:443 substreams.yaml map_graph_out -s 12369620 --debug-modules-output prepare_entity_changes

.PHONY: pack
pack:
	substreams pack ./substreams.yaml
