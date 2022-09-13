.PHONY: build-all
build-all:
	$(MAKE) -C substreams-helper build
	$(MAKE) -C uniswap-v2 build
	$(MAKE) -C erc20-price build
	$(MAKE) -C erc20-market-cap build
	$(MAKE) -C erc721 build
	$(MAKE) -C compound-v2 build
	$(MAKE) -C integrations build

.PHONY: erc20-price-spkg
erc20-price-spkg:
	cd erc20-price && make pack && mv erc20-price-substreams-v0.1.0.spkg ../target

.PHONY: integrations-darwin
integrations-darwin:
	cargo test --package substreams-integrations --target aarch64-apple-darwin -- --nocapture

.PHONY: integrations-linux
integrations-linux:
	cargo test --package substreams-integrations --target x86_64-unknown-linux-gnu -- --nocapture
