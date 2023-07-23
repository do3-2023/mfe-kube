# MFE - Kubernetes exercise

# How to deploy on k3d

1. Create a k3d cluster

```sh
k3d cluster create mfe-cluster
```

2. Create namespaces

```sh
kubectl apply -f infra/namespaces.yaml --wait
```

3. Deploy the data stack

```sh
kubectl apply -f infra/data --wait
```

4. Deploy the backend stack

```sh
kubectl apply -f infra/back --wait
```

5. Deploy the web stack

```sh
kubectl apply -f infra/web --wait
```

6. Expose the web service to your local machine (here the port `3000`)

```sh
kubectl port-forward -n mfe-web services/front-service 3000:front-port
```

Now you can access it through [localhost:3000](http://localhost:3000/)
