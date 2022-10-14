.PHONY: build-all
build-all:
	$(MAKE) -C substreams-helper build
	$(MAKE) -C uniswap-v2 build
	$(MAKE) -C erc20-holdings build
	$(MAKE) -C erc20-price build
	$(MAKE) -C erc20-market-cap build
	$(MAKE) -C erc721 build
	$(MAKE) -C compound-v2 build

.PHONY: example-all
example-all:
	$(MAKE) -C uniswap-v2 example
	$(MAKE) -C erc20-price example
	$(MAKE) -C erc20-market-cap example
	$(MAKE) -C erc721 example

.PHONY: test
test:
	$(MAKE) build-all
	$(MAKE) example-all

.PHONY: erc20-price-spkg
erc20-price-spkg:
	cd erc20-price && make pack && mv erc20-price-substreams-v0.1.0.spkg ../target