.PHONY: codegen
codegen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/ethereum,sf/substreams,google"

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: run
run:
	substreams run -e mainnet.eth.streamingfast.io:443 substreams.yaml store_chainlink_aggregator -s 10336228 -t +100

.PHONY: pack
pack:
	substreams pack ./substreams.yaml --output-file {manifestDir}/target/{spkgDefaultName}
