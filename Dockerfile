# Use the official Rust image.
# https://hub.docker.com/_/rust
FROM rustlang/rust:nightly-buster-slim

# Copy local code to the container image.
RUN USER=root cargo new --bin app
WORKDIR /app

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

COPY . .
ENV DATABASE_URL 

# Install production dependencies and build a release artifact.
RUN apt-get update
RUN apt-get upgrade -y
RUN apt-get install pkg-config libssl-dev -y
ENV ROCKET_ENV production
RUN cargo build --release

# Service must listen to $PORT environment variable.
# This default value facilitates local development.
ENV PORT 8080

# Run the web service on container startup.
ENTRYPOINT ["target/release/paiylogger"]
