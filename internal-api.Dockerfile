FROM howtocards/rust-builder:1.38 as build

WORKDIR /app

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./diesel.toml ./diesel.toml
COPY ./migrations ./migrations
COPY ./db ./db
COPY ./internal-api ./internal-api
COPY ./public-api ./public-api

RUN cargo test --release --verbose --package howtocards-internal-api

RUN cargo build --release --package howtocards-internal-api

FROM howtocards/rust-start-tools:1

WORKDIR /app

RUN touch .env

COPY --from=build /out/diesel /bin/
COPY --from=build /app/target/release/howtocards-internal-api ./

COPY --from=build /app/migrations ./migrations
COPY --from=build /app/diesel.toml ./
COPY ./build/entrypoint.sh ./entrypoint.sh

RUN chmod +x docker-entrypoint.sh && chmod +x howtocards-internal-api

ENTRYPOINT ["/app/entrypoint.sh"]
CMD ["/app/howtocards-internal-api"]
