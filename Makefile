.PHONY: build-all
build-all:
	make -C substreams-helper build
	make -C uniswap-v2 build
	make -C erc20-price build
	make -C erc721 build
	make -C compound-v2 build
