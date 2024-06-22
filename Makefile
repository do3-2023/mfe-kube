help:               ## Show this help
	@fgrep -h "##" $(MAKEFILE_LIST) | fgrep -v fgrep | sed -e 's/\\$$//' | sed -e 's/##//'

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

kube-deploy:        ## Deploy the application stack to your kube
	kubectl apply -f infra/namespaces.yaml --wait
	kubectl apply -f infra/data/ --wait
	kubectl apply -f infra/back/ --wait
	kubectl apply -f infra/web/ --wait

kube-portforward:   ## Port forward the web service to your local machine
	kubectl port-forward -n mfe-web svc/web-service 8000:web-port

kube-delete:        ## Delete the application stack from your kube
	kubectl delete -f infra/namespaces.yaml --wait
