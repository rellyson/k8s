# K8s - Complete App Deployment using Kubernetes

[![Kubernetes Badge](https://img.shields.io/static/v1?message=kubernetes&logo=kubernetes&labelColor=3e56c2&color=3e56c2&logoColor=white&label=%20)](https://kubernetes.io/)
[![Rust Badge](https://img.shields.io/static/v1?message=rustlang&logo=rust&labelColor=612f20&color=612f20&logoColor=white&label=%20)](https://www.rust-lang.org/)

## What is K8s?

K8s (an acronym to Kubernetes) is a container orchestration plataform. It started as a Google's project but became an open source project in 2014.

> "Is an open source system for managing containerized applications across multiple hosts. It provides basic mechanisms for
> deployment, maintenance, and scaling of applications."
> — Oficial Kubernetes definition at Github.

## K8s basic components

- ### Clusters

  If our a node fails, our app will be down. So a cluster is a set of nodes grouped together. Even if one node fails, the application will still be accessible from the other nodes. It's recommended to have at least 3 nodes running.

- ### Nodes

  A node is a machine either physical or virtual machine on which Kubernetes is installed. A node is a worker machine and this is where containers inside the pods will be launched by Kubernetes.

- ### Kubelets

  Kubelet is the agent that runs on each node in the cluster. The agent is responsible for making sure that the containers are running on the nodes as expected.

- ### Pods

  A pod is the smallest deployable unit that can be managed by Kubernetes. A pod is a logical group of one or more containers that share the same IP address and port space.

- ### Services

  Pods often live and die in a cluster of k8s, but we need our applications to keep running unaffected by these tragedies, so abstractions called Services were created. Each pod in a Kubernetes cluster has a specific IP, but these addresses are not exposed outside the cluster without a Service, which basically defines how a set of pods can be accessed over the network.

The oficial docs about k8s components can be found <a href="https://kubernetes.io/docs/concepts/overview/components/">here.</a>

## K8s deployment example

We have a basic Rust API that is gonna be deployed using Docker and K8s. The API code can be found in the `containers/api` folder.

- ### Step-by-step

  - **Build container image** - To be able to use the image in K8s, we first need to build a container image. To build the image using Docker, run the command:
    ```
    docker build -t rust-api-basic containers/api
    ```
  - **Tagging and pushing the image to a container registry** - We need to tag the image and push it to a container registry. In the example, we gonna push the image to Github Container Registry, using the commands:
    ```
    docker tag rust-basic-api ghcr.io/user/rust-basic-api:latest
    docker push ghcr.io/user/rust-basic-api:latest
    ```
  - **Creating a deployment to a k8s cluster** - Now that we have a container image pushed to a Container Registry, we can deploy it to a k8s cluster. For that, we created a `deployment.yaml` file, having the configurations that kubelet API needs to create a deployment. Now, we can deploy it running the command:
    ```
    kubectl apply -f k8s/deployment.yaml
    ```
  - **Creating a service in the cluster** - Now we need to create a service to access the container. For that, run the command:
    ```
    kubectl apply -f k8s/service.yaml
    ```
The application has been deployed and now we can use and visualize it using **kubectl** or a kubernetes IDE, such as **Lens**.