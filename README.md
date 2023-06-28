# MFE - Kubernetes

# How to use

1. Create a k3d cluster

```sh
k3d cluster create mfe-cluster
```

2. Apply the files

```sh
kubectl apply -f infra/web/namespace.yaml --wait
kubectl apply -f infra/web/deployment.yaml --wait
kubectl apply -f infra/web/service.yaml
```

3. Expose the web service with kubeproxy (here the port `3000`)

```sh
kubectl port-forward -n mfe-web services/nuxt-web 3000:3000-tcp
```
