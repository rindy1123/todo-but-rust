# FROM rust:1 as builder
# WORKDIR /app
# COPY . .
# RUN cargo install --path .
#
# FROM debian:buster-slim
# RUN apt-get update && rm -rf /var/lib/apt/lists/*
# COPY --from=builder /usr/local/cargo/bin/todo_api /usr/local/bin/todo_api
# COPY --from=builder /app/App.toml .
# COPY --from=builder /app/Rocket.toml .
# ENV ROCKET_ADDRESS=0.0.0.0
# EXPOSE 58000
# CMD ["todo_api"]

FROM rust:1
WORKDIR /app
EXPOSE 58000
RUN cargo install cargo-watch
CMD ["cargo", "watch", "-x", "run"]
