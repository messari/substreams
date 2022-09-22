.PHONY: build-all
build-all:
	$(MAKE) -C substreams-helper build
	$(MAKE) -C uniswap-v2 build
	$(MAKE) -C erc20-price build
	$(MAKE) -C erc721 build
	$(MAKE) -C compound-v2 build
