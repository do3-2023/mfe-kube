dev:                ## Start the dev environment
	@docker compose up -d

dev-stop:           ## Stop the dev environment
	@docker compose down

tailwind:           ## Build css file with tailwind
	cd web && \
	npx tailwindcss -c ./tailwind.config.js -i ./assets/main.css -o ./dist/main.css --minify

tailwind-watch:     ## Build css file with tailwind and watch for changes
	cd web && \
	npx tailwindcss -c ./tailwind.config.js -i ./assets/main.css -o ./dist/main.css --minify --watch

help:               ## Show this help
	@fgrep -h "##" $(MAKEFILE_LIST) | fgrep -v fgrep | sed -e 's/\\$$//' | sed -e 's/##//'
