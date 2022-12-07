.PHONY: build-all
build-all:
	echo substreams
	substreams --version
	$(MAKE) -C network build

.PHONY: run-all
run-all:
	$(MAKE) -C uniswap-v2 run
	$(MAKE) -C erc20-price run
	$(MAKE) -C erc20-market-cap run
	$(MAKE) -C erc721 run
	$(MAKE) -C network run

.PHONY: test
test:
	$(MAKE) build-all
	$(MAKE) run-all

.PHONY: install-cli
install-cli:
	cargo install --path ./messari-cli
