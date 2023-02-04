FROM docker.io/rust:1-bullseye 


ARG AWS_ACCESS_KEY_ID=
ARG AWS_SECRET_ACCESS_KEY=
ARG SCCACHE_BUCKET=
ARG SCCACHE_REGION=
ARG SCCACHE_S3_KEY_PREFIX=

ENV AWS_ACCESS_KEY_ID=$AWS_ACCESS_KEY_ID
ENV AWS_SECRET_ACCESS_KEY=$AWS_SECRET_ACCESS_KEY
ENV SCCACHE_BUCKET=$SCCACHE_BUCKET
ENV SCCACHE_REGION=$SCCACHE_REGION
ENV SCCACHE_S3_KEY_PREFIX=$SCCACHE_S3_KEY_PREFIX

ENV RUST_BACKTRACE=1 \
  CARGO_INCREMENTAL=1

RUN wget https://github.com/mozilla/grcov/releases/download/v0.8.13/grcov-x86_64-unknown-linux-musl.tar.bz2 -O /tmp/grcov.tar.bz2 \
&& tar xvfj /tmp/grcov.tar.bz2 \
&& mv /tmp/grcov /usr/local/bin/grcov \
&& chmod +x /usr/local/bin/grcov \
&& rm -rf /tmp/grcov \
&& grcov --version


# Install Doppler CLI
RUN apt-get update && apt-get install -y apt-transport-https ca-certificates curl gnupg && \
  curl -sLf --retry 3 --tlsv1.2 --proto "=https" 'https://packages.doppler.com/public/cli/gpg.DE2A7741A397C129.key' | apt-key add - && \
  echo "deb https://packages.doppler.com/public/cli/deb/debian any-version main" | tee /etc/apt/sources.list.d/doppler-cli.list 

RUN apt-get update \
  && apt-get dist-upgrade -y -o Dpkg::Options::="--force-confold" \
  && apt-get install -y --no-install-recommends \
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
  direnv \
  wget \
  doppler \
  tree \
  zip \
  unzip \
  dnsutils \ 
  lcov \
  python3 \
  python3-pip 


RUN pip3 install lcov_cobertura

RUN rustup default nightly-2023-02-02 \
  && rustup target add wasm32-unknown-unknown --toolchain nightly-2023-02-02 \
  && rustup target add x86_64-unknown-linux-gnu --toolchain nightly-2023-02-02 \
  && rustup component  add llvm-tools-preview --toolchain nightly-2023-02-02


# 0.3.0 with new s3 backend custom built
RUN wget -q  https://ipfs.anagolay.network/ipfs/bafybeifxewkbkt3ympmsjheirjrqr56znhvfjlfd3vaculn2mrdmub3cwu -O /usr/local/bin/sccache \
  && chmod +x /usr/local/bin/sccache \
  && sccache --version

ENV RUSTC_WRAPPER=/usr/local/bin/sccache

RUN cargo install $EXTRA_ARGS \
  cargo-make \
  taplo-cli \
  cargo-nextest \
  cargo-audit 
#   wasm-bindgen-cli \
#   wasm-pack \
#   cargo-chef \
#   cargo-tarpaulin \




ENV AWS_ACCESS_KEY_ID=
ENV AWS_SECRET_ACCESS_KEY=
ENV SCCACHE_BUCKET=
ENV SCCACHE_REGION=
ENV SCCACHE_S3_KEY_PREFIX=

