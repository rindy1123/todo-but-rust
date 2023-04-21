# FROM rust:1 as builder
# WORKDIR /app
# COPY . .
# RUN cargo install --path .
#
# FROM debian:buster-slim
# RUN apt-get update && rm -rf /var/lib/apt/lists/*
# COPY --from=builder /usr/local/cargo/bin/todo_api /usr/local/bin/todo_api
# ENV ROCKET_ADDRESS=0.0.0.0
# EXPOSE 48000
# CMD ["todo_api"]

# FROM rust:1
# WORKDIR /app
# COPY . .
# ENV ROCKET_ADDRESS=0.0.0.0
# EXPOSE 48000
# RUN cargo install --path .
# RUN cp /usr/local/cargo/bin/todo_api /usr/local/bin/todo_api
# CMD ["todo_api"]
