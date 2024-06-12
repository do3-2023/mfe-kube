dev:                ## Start the dev environment
	@docker compose up -d

dev-stop:           ## Stop the dev environment
	@docker compose down

help:               ## Show this help
	@fgrep -h "##" $(MAKEFILE_LIST) | fgrep -v fgrep | sed -e 's/\\$$//' | sed -e 's/##//'
