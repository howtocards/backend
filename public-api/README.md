# HowToCards

## Requirements

- [Stable rust 1.37](https://rustup.rs)
- Postgresql 11.5, requires [`libpq`](https://postgrespro.ru/docs/postgresql/9.6/libpq)

## Installation

### Configuration

```sh
cp .env.sample .env
# Edit and review .env file
# Do not forget to create user and database in local postgres
```

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
docker exec -i howtocards-public-api bash -c 'cd /app && diesel migration run'
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
