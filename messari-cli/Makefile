.PHONY: build-dagster
build-dagster:
	mkdir -p dagster-cli
	docker build -t dagster-cli:build . -f Dockerfile-Dagster
	docker container create --name extract-cli dagster-cli:build
	docker container cp extract-cli:/cli_build_folder/target/release/messari ./dagster-cli/messari
	docker container rm -f extract-cli
