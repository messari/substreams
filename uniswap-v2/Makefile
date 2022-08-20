.PHONY: codegen
codegen:
	~/go/bin/substreams protogen ./substreams.yaml --exclude-paths="sf/ethereum,sf/substreams,google"

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: stream
stream:
	substreams run -e api-dev.streamingfast.io:443 substreams.yaml map_pair -s 10008500 -t +10
