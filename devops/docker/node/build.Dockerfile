FROM sensiogroup/network-node-builder:latest 
LABEL maintainer="daniel@woss.io"
LABEL description="This is the build box."

ARG PROFILE=release

WORKDIR /sensio

COPY . /sensio

RUN export PATH="$PATH:$HOME/.cargo/bin" && \
    cargo make build

