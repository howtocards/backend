ARG RUST_VERSION=1.41.0

FROM rust:$RUST_VERSION as build

RUN USER=root cargo install diesel_cli --no-default-features --features postgres && \
    mkdir -p /out && cp $(which diesel) /out/

