# MFE - Kubernetes exercise

This project contains a simple application to be deployed on kubernetes as part of an assignment for the k8s course at Polytech Montpellier.

## Table of contents

- [MFE - Kubernetes exercise](#mfe---kubernetes-exercise)
  - [Table of contents](#table-of-contents)
  - [Repo structure](#repo-structure)
  - [How to deploy](#how-to-deploy)
  - [How to dev](#how-to-dev)
  - [License](#license)

## Repo structure

There are 5 versions branches to explore:

- [`v1`](https://github.com/do3-2023/mfe-monitoring/tree/v1) -> the initial version of the app
- [`v2`](https://github.com/do3-2023/mfe-monitoring/tree/v2) -> migrations: set `location` field to nullable | api: write, no read on `location`
- [`v3`](https://github.com/do3-2023/mfe-monitoring/tree/v3) -> migrations: no change to DB schema | api: no write, no read on `location`
- [`v4`](https://github.com/do3-2023/mfe-monitoring/tree/v4) -> migrations: delete column `location`
- [`wasm`](https://github.com/do3-2023/mfe-monitoring/tree/wasm)  -> to test deploying a service using WASI

## How to deploy

**Prerequisites:** a k8s instance, you can easily create one with kind/k3d/...

1. Deploy with:

```sh
make kube-deploy # or use the commands in the Makefile
```

2. Once the pods are all running, you can portforward the web service to access it:

```sh
make kube-portforward
```

**Note:** To stop the app, you can run `make kube-delete`.

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
