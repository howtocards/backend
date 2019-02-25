ARG RUST_VERSION=1.31.0

FROM rust:$RUST_VERSION as build

RUN USER=root cargo new --bin app
WORKDIR /app

RUN USER=root cargo install diesel_cli --no-default-features --features postgres && \
    mkdir -p /out && cp $(which diesel) /out/

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release && \
    rm src/*.rs

COPY ./ ./

RUN rm ./target/release/deps/howtocards_server* && \
    cargo build --release

FROM debian:9-slim

RUN seq 1 8 | xargs -I{} mkdir -p /usr/share/man/man{} && \
    apt update && \
    apt -y install libpq-dev postgresql-client && \
    apt clean && \
    touch .env

COPY --from=build /out/diesel /bin/
COPY --from=build /app/target/release/howtocards_server ./
COPY docker-entrypoint.sh ./entrypoint.sh
RUN chmod +x entrypoint.sh && chmod +x howtocards_server

ENTRYPOINT ["/entrypoint.sh"]
CMD ["/howtocards_server"]
