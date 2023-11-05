FROM rust:1.73.0-alpine as builder
WORKDIR /usr/src/discord-rust
COPY . ./
RUN apk update && apk add git && apk add make && apk add musl-dev && apk add cmake 
RUN cargo install --path .
RUN cargo build --release

FROM debian:bullseye as runtime
COPY --from=builder /usr/src/discord-rust/target/release/discord-bot-rs /usr/local/bin
RUN apt-get update && apt-get install -y cmake make && rm -rf /var/lib/apt/lists/*
# RUN apt-get update & rm -rf /var/lib/apt/lists/*
# COPY --from=builder /usr/local/cargo/bin/discord-rust /usr/local/bin/discord-rust

CMD [ "/usr/local/bin/discord-bot-rs" ]