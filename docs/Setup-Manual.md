# Manual Setup

If you want to run the API and it's dependencies manually here's how to do it.

## Prepare the system 
```sh
$ apt update && apt upgrade -y
```

## Install dependencies
```sh
$ apt install -y \
  curl \
  git \
  openssl \
  libssl-dev \
  pkg-config \
  ruby-full \
  libfreetype-dev \
  build-essential \
  gcc \
  libffi-dev \
  fontconfig \
  libfontconfig-dev \
```

## Install rust
```sh
curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
```

## Clone and Build iboardbot API Server
```sh
$ cd ~
$ git clone https://github.com/Stay1444/iboardbot
$ cd iboardbot
$ cargo build --release
```

The binary will be in `./iboardbot/target/release/iboardbot`

## Clone and Build svg2gcode - Needed for drawing svgs
```sh
$ git clone https://github.com/sameer/svg2gcode
$ cd svg2gcode
$ cargo build --release
```

## Install text2svg - Needed for writing text to the board
```sh
$ gem install text2svg
```

## Running

```sh
$ ./iboardbot --port 8080
```

[Adding it as a SystemD service](./Adding-SystemD-Service.md)

## Fonts

TTF fonts used for text rendering go under the `./fonts` directory.

## Board configuration

Board configurations, such as dimensions, go on the `./boards` directory.