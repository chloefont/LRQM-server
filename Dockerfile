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
ENV SQLX_OFFLINE=true

# Install SQLX
RUN cargo install sqlx-cli --no-default-features --features postgres

# Build the application in release mode
RUN cargo build --release 

# Use a minimal base image for the final executable
FROM debian:bullseye-slim
ARG APP=/app
ENV APP_USER=appuser

WORKDIR ${APP}

RUN groupadd $APP_USER
RUN useradd -g $APP_USER $APP_USER
RUN mkdir -p ${APP}

ENV APP_USER=appuser

COPY ./migrations ./migrations
COPY ./start.sh start.sh
RUN chmod +x start.sh

COPY --from=builder /app/target/release/lrqm_server lrqm_server
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

EXPOSE 3000/tcp
CMD ["./start.sh"]
