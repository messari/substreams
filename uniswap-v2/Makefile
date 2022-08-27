.PHONY: codegen
codegen:
	~/go/bin/substreams protogen ./substreams.yaml --exclude-paths="sf/ethereum,sf/substreams,google"

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: example
example:
	substreams run -e api-dev.streamingfast.io:443 substreams.yaml map_pair_created_event,map_pools,store_pools -s 10008355 -t +200
