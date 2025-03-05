# lrqm_server

## Installation

Install Rustup
```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Copy the `.env.vars` file to `.env` and fill in the values.
```bash
cp .env.var .env
```

Install the cargo dependencies
```bash
cargo build
```

Create the database if it doesn't exist
```bash
sqlx database create
```

Run the migrations
```bash
sqlx migrate run
```

## Running

Run the docker-compose file
```bash
docker-compose up
```

Run the server
```bash
cargo run
```
