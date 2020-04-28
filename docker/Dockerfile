FROM phusion/baseimage:0.11 as builder
LABEL maintainer "daniel@woss.io"
LABEL description="This is the build stage for SensioNetwork. Here we create the binary."

ARG PROFILE=release
WORKDIR /sensio

COPY . /sensio

RUN apt-get update && \
  apt-get upgrade -y && \
  apt-get install -y cmake pkg-config libssl-dev git clang
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
  export PATH=$PATH:$HOME/.cargo/bin && \
  scripts/init.sh && \
  cargo build --$PROFILE

# ===== SECOND STAGE ======

FROM phusion/baseimage:0.11
LABEL maintainer "daniel@woss.io"
LABEL description="This is the 2nd stage: a very small image where we copy the SensioNetwork binary."
ARG PROFILE=release

RUN mv /usr/share/ca* /tmp && \
  rm -rf /usr/share/*  && \
  mv /tmp/ca-certificates /usr/share/ && \
  rm -rf /usr/lib/python* && \
  useradd -m -u 1000 -U -s /bin/sh -d /sensio sensio && \
  mkdir -p /sensio/.local/share/sensio && \
  chown -R sensio:sensio /sensio/.local && \
  ln -s /sensio/.local/share/sensio /data && \
  rm -rf /usr/bin /usr/sbin

COPY --from=builder /sensio/target/$PROFILE/sensio /usr/local/bin

# checks
RUN ldd /usr/local/bin/sensio && \
  /usr/local/bin/sensio --version

# Shrinking
RUN rm -rf /usr/lib/python* && \
  rm -rf /usr/bin /usr/sbin /usr/share/man

USER sensio

EXPOSE 30333 9933 9944 9615

VOLUME ["/data"]

CMD ["/usr/local/bin/sensio"]
