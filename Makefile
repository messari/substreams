.PHONY: build-all
build-all:
	$(MAKE) -C substreams-helper build
	$(MAKE) -C uniswap-v2 build
	$(MAKE) -C erc20-holdings build
	$(MAKE) -C erc20-price build
	$(MAKE) -C erc20-market-cap build
	$(MAKE) -C erc721 build
	$(MAKE) -C compound-v2 build

.PHONY: erc20-price-spkg
erc20-price-spkg:
	cd erc20-price && make pack && mv erc20-price-substreams-v0.1.0.spkg ../target