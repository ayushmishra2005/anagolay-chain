# syntax=docker/dockerfile:experimental
FROM rust:1.51.0-slim-buster as base
LABEL maintainer="daniel@woss.io" 
LABEL description="Debian 10 (buster) based Rust (1.51.0) base image for substrate builds. Use this image as a build image in the multistage build process. This image comes with all the needed libraries for the successful build of the most used libraries for the substrate. This image does not expose any ports, it just prepares the environment for the build."

LABEL "network.anagolay.vendor"="Anagolay Network"

# rust image defaults, for the ref
# ENV RUSTUP_HOME=/usr/local/rustup \
#     CARGO_HOME=/usr/local/cargo \
#     PATH=/usr/local/cargo/bin:$PATH \
#     RUST_VERSION=1.51.0

WORKDIR /app

RUN mkdir -p .cache/sccache

# Env variables that help us with the running the image. This is available in the container
ENV CACHE_HOME=/app/.cache
ENV SCCACHE_DIR=/app/.cache/sccache

# Arguments we need to build an image
ARG RUST_TOOLCHAIN=nightly-2021-03-25

COPY ./scripts scripts
COPY ./rust-toolchain .


RUN bash ./scripts/00-upgrade-system.sh
RUN bash ./scripts/01-install-system-dependencies.sh
RUN bash ./scripts/02-setup-toolchain.sh $RUST_TOOLCHAIN

# this is done like this so we can install the cache and then assign the wrapper so the cargo build can pick it up and other cargo compiler
RUN cargo install sccache

ENV RUSTC_WRAPPER=$CARGO_HOME/bin/sccache

RUN bash ./scripts/03-install-cargo-pacakges.sh
