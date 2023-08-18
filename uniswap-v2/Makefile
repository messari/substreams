.PHONY: build_all
build_all:
	$(MAKE) -C ../erc20-price build
	$(MAKE) -C ../erc20-price pack
	cargo build --target wasm32-unknown-unknown --release

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: run_mainnet
run_mainnet:
	substreams run -e mainnet.eth.streamingfast.io:443 substreams.yaml graph_out -s 10008300

.PHONY: run
run:
	substreams run -e bsc.substreams.pinax.network:9000 substreams.yaml graph_out -s 30907417

.PHONY: pack
pack:
	substreams pack ./substreams.yaml
