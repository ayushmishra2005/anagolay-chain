#!/usr/bin/env bash
echo "***** INSTALLING DEPS *****"

set -o errexit

apt-get update
# && apt-get dist-upgrade -y -o Dpkg::Options::="--force-confold" \
apt-get install -y --no-install-recommends \
  git \
  libssl-dev \
  clang \
  cmake \
  libclang-dev \
  musl-tools \
  libffi-dev \
  pkg-config \
  gcc \
  build-essential \
  direnv
