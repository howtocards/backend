ARG RUST_VERSION=1.31.0

FROM rust:$RUST_VERSION as build

RUN USER=root cargo new --bin app
WORKDIR /app

RUN USER=root cargo install diesel_cli --no-default-features --features postgres
RUN mkdir -p /out && cp $(which diesel) /out/

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./ ./

RUN rm ./target/release/deps/howtocards_server*
RUN cargo build --release

FROM debian:9-slim

RUN apt-get update && apt-get -y install libpq-dev && touch .env

COPY --from=build /out/diesel /bin/
COPY --from=build /app/target/release/howtocards_server ./

ENTRYPOINT ["./howtocards_server"]
