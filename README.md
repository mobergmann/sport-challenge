# Container
To create a container you need to follow these steps:
```
cargo build --release
podman build -t sport-challenge-image .
```
If you are using another container frontend, like docker, change podman to docker.

If you want to run the container, enter the following command:
```
podman run -d -p <host-port>:3000 --name sport-challenge localhost/sport-challenge-image
```
