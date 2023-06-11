.PHONY: build-all
build-all:
	$(MAKE) -C substreams-helper build
	$(MAKE) -C uniswap-v2 build
	$(MAKE) -C erc20-holdings build
	$(MAKE) -C erc20-price build
	$(MAKE) -C erc20-market-cap build
	$(MAKE) -C erc721 build
	$(MAKE) -C compound-v2 build
	$(MAKE) -C network build
	$(MAKE) -C solana-sample build
	$(MAKE) -C eth-balance build
	$(MAKE) -C ens-names build
	$(MAKE) -C eth-supply build

.PHONY: run-all
run-all:
	$(MAKE) -C uniswap-v2 run
	$(MAKE) -C erc20-price run
	$(MAKE) -C erc20-market-cap run
	$(MAKE) -C erc721 run
	$(MAKE) -C network run
	$(MAKE) -C ens-names run
	$(MAKE) -C eth-supply run

.PHONY: test
test:
	$(MAKE) build-all
	$(MAKE) run-all

.PHONY: install-cli
install-cli:
	cargo install --path ./messari-cli

.PHONY: upload-cli-for-dagster
upload-cli-for-dagster:
	$(MAKE) -C messari-cli build-dagster
	messari upload-file-to-spkg-bucket ./messari-cli/dagster-cli/messari

.PHONY: pack-all
pack-all:
	$(MAKE) -C erc20-price pack

.PHONY: upload-all
upload-all:
	$(MAKE) -C erc20-price upload

