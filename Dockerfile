FROM rust:1.29.1 as build

RUN USER=root cargo new --bin howtocards
RUN touch /howtocards/src/lib.rs
WORKDIR /howtocards

# COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm -rf src

COPY ./src ./src
COPY ./migrations ./migrations
COPY ./diesel.toml ./diesel.toml

RUN rm -rf ./target/release/libhowtocards.*
RUN rm -rf ./target/release/howtocards_server
RUN rm -rf ./target/release/deps/*howtocards*
RUN cargo build --release

FROM rust:1.29.1

RUN ls -la /
RUN apt-get update && apt-get install libpq-dev

RUN touch .env
COPY --from=build /howtocards/target/release/howtocards_server .

CMD ["./howtocards_server"]
