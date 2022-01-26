#!/usr/bin/env bash

set -e
set -x

echo "*** Updating system and instaling dependencies ***"

apt-get update &&
    apt-get install -y --no-install-recommends \
        libssl-dev \
        clang \
        cmake \
        libclang-dev \
        musl-tools \
        libffi-dev 
