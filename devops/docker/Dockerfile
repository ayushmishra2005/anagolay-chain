# syntax=docker/dockerfile:experimental
# Build box for all packages
# this image does not contain the files or any env vars
FROM rust:1 as builder
LABEL maintainer="daniel@woss.io" 
LABEL description="This box is HUGE, it contains the cached verison of the node and its code"

ARG RUST_TOOLCHAIN=nightly-2020-10-06
ENV CARGO_TERM_COLOR=always
ENV RUST_BACKTRACE=1


ENV CC gcc
ENV CXX g++

WORKDIR /anagolay

COPY . /anagolay

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
    cmake \
    pkg-config \
    libssl-dev \
    git \
    clang \
    apt-utils \
    gcc \
    build-essential \
    libclang-dev \
    && rm -rf /var/lib/apt/lists/*

RUN rustup install ${RUST_TOOLCHAIN} \
    && rustup default ${RUST_TOOLCHAIN} \
    && rustup target add wasm32-unknown-unknown

WORKDIR /app
COPY . .

RUN cargo fetch
RUN cargo build --release

# Final stage. Copy the node executable and the script
FROM debian:stretch-slim

ENV RUST_BACKTRACE 1

COPY --from=builder /app/target/release/sensio /usr/local/bin

# let's see will this run in this container
RUN /usr/local/bin/sensio --version

USER sensio

EXPOSE 30333 9933 9944

VOLUME ["/data"]

ENTRYPOINT  ["/usr/local/bin/sensio"]
CMD []
