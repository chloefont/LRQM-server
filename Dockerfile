FROM rust:slim-bullseye AS builder

# Set working directory
WORKDIR /app

# Copy Cargo files separately to leverage Docker caching
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./.env ./.env
COPY ./src ./src
COPY ./migrations ./migrations

# Important, to build without the DB running
COPY ./.sqlx ./.sqlx
ENV SQLX_OFFLINE true

# Build the application in release mode
RUN cargo build --release 

# Use a minimal base image for the final executable
FROM debian:bullseye-slim

WORKDIR /app

COPY ./migrations ./migrations
COPY --from=builder /app/target/release/lrqm_server lrqm_server

EXPOSE 3000/tcp
CMD ["/app/lrqm_server"]
