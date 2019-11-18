FROM howtocards/rust-builder:1.38 as build

ARG CRATE_NAME

WORKDIR /app

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./diesel.toml ./diesel.toml
COPY ./migrations ./migrations
COPY ./db ./db
COPY ./internal-api ./internal-api
COPY ./public-api ./public-api

RUN cargo test --release --verbose --package howtocards-$CRATE_NAME

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
