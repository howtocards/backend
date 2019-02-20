# HowToCards

## Requirements

- [Stable rust](https://rustup.rs)
- Postgresql 10, requires [`libpq`](https://postgrespro.ru/docs/postgresql/9.6/libpq)

## Installation

Install [`diesel_cli`]

```sh
apt install gcc # ubuntu

cargo install diesel_cli --no-default-features --features postgres
```

## Build and run

```sh
# Build production binary
cargo build --release

# Development
cargo install cargo-watch
cargo watch -x run
```

## After pull, checkout, or db change

```sh
diesel migration run
```

To revert migration run

```sh
diesel migration revert
```
