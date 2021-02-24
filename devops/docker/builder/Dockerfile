FROM rust:1.46.0
LABEL maintainer="daniel@woss.io" 
LABEL description="This box is HUGE, it contains the cached verison of the node and its code"
# ENV DEBIAN_FRONTEND=noninteractive
ENV RUST_BACKTRACE 1

WORKDIR /anagolay

COPY . /anagolay

RUN apt-get update && \
    # apt-get dist-upgrade -y -o Dpkg::Options::="--force-confold" && \
    apt-get install -y cmake pkg-config libssl-dev git clang apt-utils cmake pkg-config libssl-dev git gcc build-essential clang libclang-dev

# RUN curl https://sh.rustup.rs -sSf | sh -s -- -y 

# RUN echo 'export PATH="$PATH:$HOME/.cargo/bin"' >> ~/.bashrc && \
#     /bin/bash -c "source ~/.bashrc"
# export PATH="$PATH:$HOME/.cargo/bin" && \
RUN ./devops/scripts/init.sh && \
    cargo install cargo-make && \ 
    cargo make build

RUN cargo version

RUN ./target/release/anagolay --version
