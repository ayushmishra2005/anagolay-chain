#!/usr/bin/env bash

set -e
set -x

echo "*** Updating system and instaling dependencies ***"

apt-get update &&
    apt-get install -y --no-install-recommends \
        git \
        build-essential \
        pkg-config \
        libssl-dev \
        clang \
        g++ \
        cmake \
        libclang-dev \
        musl-tools \
        ccache \
        lcov \
        python3-pip \
        libffi-dev \
        python3-dev \
        python3-setuptools

pip3 install wheel
pip3 install lcov_cobertura
