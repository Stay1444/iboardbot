# Docker Setup

## Requirements
- Git
- Curl

## Installing Docker 

You can skip this step if you already have docker installed.

I like to use the [get docker](https://get.docker.com/) script but you can also follow the [official guide](https://docs.docker.com/engine/install/debian/).

```sh
$ curl -fsSL https://get.docker.com -o install-docker.sh
$ sudo sh install-docker.sh
```

Then you can enable the service in systemd so it starts and is started everytime the system boots

```sh
$ sudo systemctl enable --now docker
```

And that's it! You can verify that it's working by running:

```sh
$ sudo docker ps
```

## Cloning the repository & building the docker image

First clone the repo

```sh
$ git clone https://github.com/Stay1444/iboardbot
$ cd iboardbot
```

Then we can build the docker image

```sh
$ sudo docker build . -t iboardbot:latest
```

Now wait a bit and if everything has gone correctly it should say something similar to this

```
Successfully built 83e82e2f8f9b
Successfully tagged iboardbot:latest
```

## Run

```sh
$ mkdir -p iboardbot/fonts iboardbot/boards # create required directories

$ docker run -d \
  --restart=always \  # Restart the container automatically if it exits
  -p 8080:80 \  # Map port 8080 on the host to port 80 in the container
  -v ./iboardbot/fonts:/app/fonts \  # Mount the host's /path/to/fonts to /app/fonts in the container
  -v ./iboardbot/boards:/app/boards \  # Mount the host's /path/to/boards to /app/boards in the container
  --detach \  # Run the container in detached mode (background)
  iboardbot:latest
```

Note that if you want to change the port you should change the `8080` value, leave the rightside `80` alone as that is the port mapping for *inside* of the docker container. If you don't know what I mean I recommend reading [the Docker docs](https://docs.docker.com/network/#published-ports).

# Final notes

Remember to [reconfigure your board.](./Flash-Arduino.md)