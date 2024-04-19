# Manual Setup

If you want to run the API and it's dependencies manually here's how to do it.

## Dependencies

- Git
- [Rust + Cargo](https://www.rust-lang.org/tools/install)
- Gem / Ruby
- libssl / openssl

Dependencies that I will explain how to get below:

- svg2gcode - Needed for drawing svgs
- text2svg - Needed for writing text to the board

## Building

Clone the repository

```sh
$ git clone https://github.com/Stay1444/iboardbot
$ cd iboardbot
```

Build the project

```sh
$ cargo build --release
```

You will find the binary in `./target/release/iboardbot`.

## Running

Once you have the binary you can simply invoke it `./iboardbot` and it will run.

## Installing extra dependencies (svg2gcode + text2svg)

The following third party programs are used to generate the necessary stuff to draw and render text and svg images.

### svg2gcode

Clone

```sh
$ git clone https://github.com/sameer/svg2gcode
$ cd svg2gcode
```

Now build it

```sh
$ cargo build --release
```

Once build you'll need to either copy `./target/release/svg2gcode` to
- A folder within your `$PATH`, so for example `/usr/bin/` or `~/bin`.
- The working directory from where you'll run the `iboardbot` API.

### text2svg

You'll also need https://github.com/ksss/text2svg. You can install it using `gem` like this

```sh
$ gem install text2svg
```

That's it!

## Fonts

TTF fonts used for text rendering go under the `./fonts` directory.

## Board configuration

Board configurations, such as dimensions, go on the `./boards` directory.