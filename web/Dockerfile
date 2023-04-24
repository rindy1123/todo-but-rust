FROM rust:1-slim-bullseye
WORKDIR /app
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk
