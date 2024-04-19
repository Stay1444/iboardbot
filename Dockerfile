FROM rust AS rust-builder

RUN mkdir /app
WORKDIR /app

COPY . .
RUN cargo build --release

WORKDIR /
RUN git clone https://github.com/sameer/svg2gcode
WORKDIR /svg2gcode
RUN cargo build --release

FROM debian

RUN apt-get update && apt-get install -y openssl libssl-dev ruby-full libfreetype-dev build-essential gcc libffi-dev ruby-dev
RUN mkdir /app

COPY --from=rust-builder /app/target/release/iboardbot /usr/bin/iboardbot
RUN chmod +x /usr/bin/iboardbot

COPY --from=rust-builder /svg2gcode/target/release/svg2gcode /usr/bin/svg2gcode

RUN gem install text2svg

WORKDIR /app

EXPOSE 80

VOLUME [ "/app/fonts", "/app/boards" ]

ENTRYPOINT [ "/usr/bin/iboardbot", "--port", "80" ]
