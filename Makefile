

help:
	@echo ======================================================================================
	@fgrep -h "##" $(MAKEFILE_LIST) | fgrep -v fgrep | sed -e 's/\\$$//' | sed -e 's/##//'
	@echo ======================================================================================


image-solana:			## build solana image
	@./devops/ecr/buildx-image solana

image-solana-local:		## build solana image for local terminal
	@./devops/tools/build-image \
		local \
		quantmind-solana-local \
		--build-arg USER=$(USER)

solana:				## enter solana image
	@make image-solana-local
	@./devops/images/local/run


build:				## build solana programs
	cargo build
