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
	substreams run substreams.yaml map_events -s 12274555
.PHONY: pack
pack:
	substreams pack ./substreams.yaml
