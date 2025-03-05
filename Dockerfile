FROM rust:slim-bullseye AS builder

## Source: https://www.reddit.com/r/rust/comments/1he3woc/sqlx_in_docker_container/

# Set working directory
WORKDIR /app

# Copy Cargo files separately to leverage Docker caching
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./.env.var ./.env
COPY ./src ./src
COPY ./.sqlx ./.sqlx
COPY ./migrations ./migrations

# Build the application in release mode
RUN cargo build --release 

# Use a minimal base image for the final executable
FROM debian:bullseye-slim

WORKDIR /app

COPY ./migrations ./migrations
COPY --from=builder /app/target/release/lrqm_server lrqm_server

EXPOSE 3000/tcp
CMD ["/app/lrqm_server"]
