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

## Running

Run the docker-compose file
```bash
docker-compose up
```

Run the server
```bash
cargo run
```
