### CLUSTER COMMANDS
cluster-create:     ## Create a basic cluster through k3d
	@k3d cluster create mfe-cluster

cluster-delete:     ## Delete the cluster created with the command `create-cluster`
	@k3d cluster delete mfe-cluster

### APP STACK (k8s) COMMANDS
stack-apply:        ## Deploy the app stack to current cluster
	@kubectl apply -f infra/namespaces.yaml
	@kubectl apply -f infra/data
	@kubectl apply -f infra/back
	@kubectl apply -f infra/web

stack-delete:       ## Delete the app stack from current cluster
	@kubectl delete -f infra/namespaces.yaml

stack-expose:       ## Expose the web service to port 3000
	@kubectl port-forward -n mfe-web services/web-service 3000:web-port

### OTHERS
dev:                ## Start the dev environment
	@docker compose up -d

dev-stop:           ## Stop the dev environment
	@docker compose down

help:               ## Show this help
	@fgrep -h "##" $(MAKEFILE_LIST) | fgrep -v fgrep | sed -e 's/\\$$//' | sed -e 's/##//'
