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
	substreams run -e bnb.streamingfast.io:443 substreams.yaml graph_out -s 6809737

.PHONY: pack
pack:
	substreams pack ./substreams.yaml
