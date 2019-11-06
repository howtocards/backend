ARG RUST_VERSION=1.38.0

FROM rust:$RUST_VERSION as build

WORKDIR /app

RUN USER=root cargo install diesel_cli --no-default-features --features postgres && \
    mkdir -p /out && cp $(which diesel) /out/

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./migrations ./migrations
COPY ./diesel.toml ./diesel.toml
COPY ./internal-api ./internal-api
COPY ./db ./db
COPY ./public-api/Cargo.toml ./public-api/Cargo.toml

RUN cargo test --release --verbose --package howtocards-internal-api

RUN cargo build --release

FROM debian:9-slim

RUN seq 1 8 | xargs -I{} mkdir -p /usr/share/man/man{} && \
    apt update && \
    apt -y install libpq-dev postgresql-client && \
    apt clean && \
    touch .env

WORKDIR /app

COPY --from=build /out/diesel /bin/
COPY --from=build /app/target/release/howtocards-internal-api ./

COPY --from=build /app/internal-api ./internal-api
COPY --from=build /app/migrations ./migrations
COPY --from=build /app/diesel.toml ./
COPY docker-entrypoint.sh ./entrypoint.sh

RUN chmod +x entrypoint.sh && chmod +x howtocards-internal-api

ENTRYPOINT ["/app/entrypoint.sh"]
CMD ["/app/howtocards-internal-api"]
