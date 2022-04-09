

help:
	@echo ======================================================================================
	@fgrep -h "##" $(MAKEFILE_LIST) | fgrep -v fgrep | sed -e 's/\\$$//' | sed -e 's/##//'
	@echo ======================================================================================


image-solare:			## build solare image
	@./devops/github/build-image solare solare
	@./devops/tools/build-image \
		local \
		quantmind-solare-local \
		--build-arg USER=$(USER)

solare:				## enter solare image
	@./devops/images/local/run-bash


build:				## build solana programs
	@./devops/images/local/run anchor build


config-local:			## Configue solana client to use local test net
	@./devops/images/local/run solana-local

config-dev:			## Configue solana client to use devnet
	@./devops/images/local/run solana-dev

config-prod:			## Configue solana client to use mainnet
	@./devops/images/local/run solana-prod


deploy:				## build solana programs
	@./devops/images/local/run anchor deploy


install-solana:			## install solana SDK
	@./devops/images/solare/install/solana

install-avm:			## install anchor version manager
	cargo install --git https://github.com/project-serum/anchor avm --locked --force


install-anchor:			## install anchor globally
	avm install latest
	avm use latest


solana-test-validator:		## run the test validator
	docker-compose up -d


solana-test-validator-stop:	## stop the test validator
	docker-compose down


tests:				## run tests
	@echo "nothing to do"


version:			## print solana version in solare image
	@./devops/images/solare/run solana --version
