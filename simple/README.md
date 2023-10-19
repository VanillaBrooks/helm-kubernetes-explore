# Simple 

A single webserver serving a static HTML page on 8080

## Building

https://stackoverflow.com/questions/56392041/getting-errimageneverpull-in-pods

```
eval $(minikube docker-env)
sudo usermod -aG docker $USER && newgrp docker
# do not use sudo here
docker build . -t simple-webservice:0.3.0
```

## Deploying

```
helm install simple-chart/ --generate-name
```

```
helm list
```

then, you have to manually port forward the chart to the host:

```
kubectl port-forward service/simple-chart-1697599109 8080:8080
```

## Chart Diff

```diff
```
