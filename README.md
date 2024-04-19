# IBoardBot API

Recreated IBoardBot API in Rust.

![board with globe, linux tux and atom drawn](./images/img0.jpg)

# How to run it

[Manual setup](./docs/Setup-Manual.md)

[Docker setup (recommended)](./docs/Setup-Docker.md)

[Reconfiguring the board](./docs/Flash-Arduino.md)

# Adding fonts

The `./fonts` directory is used to draw text on the board. You can specify which `ttf` font to use on the JSON request. It will use a default one if no font is specified.

# Board Configuration

The first time that a board connects to the API a configuration file for that board will be created under `./boards`. If you named your board `main`, the configuration file will be `./boards/main.yaml`

In this file you can specify the board dimensions (size). Ive found that for the board that is shown on the images, `3500` (width) X `1000` (height) works fine.

# API Docs

You can find the API docs at the `/docs` endpoint in the API.

# Frontend / Easy to use app

At the moment the only real way to use it is sending HTTP requests to the API, which you can find the docs on the `/docs` endpoint. 

I'm working on an easy to use frontend so be patient for that. 