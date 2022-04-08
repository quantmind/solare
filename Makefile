

help:
	@echo ======================================================================================
	@fgrep -h "##" $(MAKEFILE_LIST) | fgrep -v fgrep | sed -e 's/\\$$//' | sed -e 's/##//'
	@echo ======================================================================================


image-solare:			## build solare image
	@./devops/github/buildx-image solare solare

image-solare-local:		## build solare image for local terminal
	@./devops/tools/build-image \
		local \
		quantmind-solare-local \
		--build-arg USER=$(USER)

solare:				## enter solare image
	@make image-solare-local
	@./devops/images/local/run


build:				## build solana programs
	cargo build


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
