# It takes a long when build is really first time.

# Build Stage 1: Build dependencies with cargo chef
FROM rust as cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Build Stage 2: Build with cache
FROM rust as builder
COPY . /app
WORKDIR /app
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release

# Build Stage 3: Deploy
FROM debian:latest
RUN apt-get update
RUN apt-get install -y libssl3 libssl-dev openssl ca-certificates && update-ca-certificates && apt clean && rm -rf /var/lib/apt/lists/*
#  RUN ln -s libssl.so.3 libssl.so.1.1 && ldconfig /usr/local/lib64/

# Change dir name from "testApp" to real name
COPY --from=builder /app/target/release/app /app/projects_notifier
WORKDIR /app

# Start (Change binary name from "testApp" to real name)
CMD ["./projects_notifier"]