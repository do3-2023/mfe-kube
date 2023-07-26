# MFE - Kubernetes exercise :speech_balloon:

This project contains a chat application to be deployed on kubernetes as part of an assignment for the Orchestration course at Polytech Montpellier.

<details>
    <summary><strong>Click to see a demo of the app</strong></summary>

[app demo](https://github.com/do3-2023/mfe-kube/assets/22498591/f1abac17-2e19-4914-90c4-cb7c2a7cb43f)

</details>

<details>
    <summary><strong>Click to see the assignement instructions</strong></summary>

For this assignment, we were given these instructions:

<img src="https://github.com/do3-2023/mfe-kube/assets/22498591/f97bed83-f553-4521-b43f-78e848a93a4e" alt="guidelines" width="850" />

</details>

## Table of contents

- [Deploy on k3d `Makefile`](#deploy-on-k3d-makefile)
- [Deploy on k3d `manually`](#deploy-on-k3d-manually)
- [How to dev](#how-to-dev)

## Deploy on k3d `Makefile`

__Requirements:__ [k3d](https://k3d.io/)

1. __Create__ a k3d cluster <small>(skip if you've already a cluster)</small>

```sh
make cluster-create
```

2. __Apply__ the config

```sh
make stack-apply
```

3. __Expose__ the web port

```sh
make stack-expose
```

Now it's done, you just have to wait a few moments for
the app to deploy properly.

You should be able to access it here: [localhost:3000](http://localhost:3000/)

4. __Delete__ the cluster when you've finished

```sh
make cluster-delete
```

Or you could just delete the app from the cluster with

```sh
make stack-delete
```

## Deploy on k3d `manually`

1. Create a k3d cluster

```sh
k3d cluster create mfe-cluster
```

2. Create namespaces

```sh
kubectl apply -f infra/namespaces.yaml
```

3. Deploy the data stack

```sh
kubectl apply -f infra/data
```

4. Deploy the backend stack

```sh
kubectl apply -f infra/back
```

5. Deploy the web stack

```sh
kubectl apply -f infra/web
```

6. Expose the web service to your local machine (here the port `3000`)

```sh
kubectl port-forward -n mfe-web services/web-service 3000:web-port
```

Now you can access it through: [localhost:3000](http://localhost:3000/)  
You may have to wait a few moments for it to start properly

7. You can now delete the cluster when you're done

```sh
k3d cluster delete mfe-cluster
```

Or only delete the resources created with

```sh
kubectl delete -f infra/namespaces.yaml
```

## How to dev

1. You can start the dev environment with

```sh
make dev # or docker compose up -d
```

2. When you're done, stop it with

```sh
make dev-stop # or docker compose down
```

## License

[MIT](./LICENSE)
