FROM howtocards/rust-builder:1.39 as build

ENV USER="root"
WORKDIR /app

COPY ./Cargo.lock ./Cargo.toml ./
RUN cargo new public-api --bin --name howtocards-public-api && \
  cargo new internal-api --bin --name howtocards-internal-api && \
  cargo new db --lib --name howtocards-db
COPY ./internal-api/Cargo.toml ./internal-api/Cargo.toml
COPY ./public-api/Cargo.toml ./public-api/Cargo.toml
COPY ./db/Cargo.toml ./db/Cargo.toml
RUN cargo build --release

RUN find ./target -type f -name *howtocards* | xargs rm -rf

COPY ./diesel.toml ./diesel.toml
COPY ./migrations ./migrations
COPY ./db ./db
COPY ./internal-api ./internal-api
COPY ./public-api ./public-api

ARG CRATE_NAME

# RUN cargo test --release --verbose --package howtocards-$CRATE_NAME

RUN cargo build --release --package howtocards-$CRATE_NAME

# ----------------------------------------------------------------

FROM howtocards/rust-start-tools:1

ARG CRATE_NAME

WORKDIR /app

RUN touch .env

COPY --from=build /out/diesel /bin/
COPY --from=build /app/target/release/howtocards-$CRATE_NAME ./server

COPY --from=build /app/migrations ./migrations
COPY --from=build /app/diesel.toml ./
COPY ./docker-entrypoint.sh ./entrypoint.sh

RUN chmod +x entrypoint.sh && chmod +x server

ENTRYPOINT ["/app/entrypoint.sh"]
CMD ["/app/server"]
