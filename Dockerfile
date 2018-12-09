ARG RUST_VERSION=1.31.0
ARG ALPINE_VERSION=3.8

FROM rust:$RUST_VERSION as build

RUN USER=root cargo new --bin /app
WORKDIR /app

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release && \
    rm -rf src

COPY ./ ./

RUN rm -rf ./target/release/howtocards_server && \
    cargo build --release

FROM debian:9-slim

RUN apt-get update && apt-get -y install libpq-dev && touch .env

COPY --from=build /app/target/release/howtocards_server ./

ENTRYPOINT ["/howtocards_server"]
