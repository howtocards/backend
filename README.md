# HowToCards

## Requirements

- [Stable rust](https://rustup.rs)
- Postgresql 11, requires [`libpq`](https://postgrespro.ru/docs/postgresql/9.6/libpq)

## Installation

### Ubuntu

```sh
curl https://sh.rustup.rs -sSf | sh
sudo apt install gcc
sudo apt install postgresql postgresql-contrib libpq-dev
```

### Docker
Using makefile
```sh
make docker-init
```
or manual
```sh
docker-compose up -d
docker exec -i howtocards_backend bash -c 'cd /app && diesel migration run'
```

### Diesel CLI

```sh
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
