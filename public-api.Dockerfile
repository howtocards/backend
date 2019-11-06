ARG RUST_VERSION=1.38.0

FROM rust:$RUST_VERSION as build

WORKDIR /app

RUN USER=root cargo install diesel_cli --no-default-features --features postgres && \
    mkdir -p /out && cp $(which diesel) /out/

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./migrations ./migrations
COPY ./diesel.toml ./diesel.toml
COPY ./public-api ./public-api

RUN cargo test --release --verbose --all

RUN cargo build --release

FROM debian:9-slim

RUN seq 1 8 | xargs -I{} mkdir -p /usr/share/man/man{} && \
    apt update && \
    apt -y install libpq-dev postgresql-client && \
    apt clean && \
    touch .env

WORKDIR /app

COPY --from=build /out/diesel /bin/
COPY --from=build /app/target/release/howtocards-public-api ./

COPY --from=build /app/public-api ./public-api
COPY --from=build /app/migrations ./migrations
COPY --from=build /app/diesel.toml ./
COPY docker-entrypoint.sh ./entrypoint.sh

RUN chmod +x entrypoint.sh && chmod +x howtocards-public-api

ENTRYPOINT ["/app/entrypoint.sh"]
CMD ["/app/howtocards-public-api"]
