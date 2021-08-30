####### BUILDER image
FROM woss/substrate_rust_ci_cachepot as builder

LABEL maintainer="daniel@woss.io" 
LABEL description="This is the build stage for Anagolay. Here we create the binary."

ARG PROFILE=release
ARG AWS_ACCESS_KEY_ID
ARG AWS_SECRET_ACCESS_KEY
ARG CACHEPOT_BUCKET
ARG CHEPOT_S3_KEY_PREFIX

ENV AWS_ACCESS_KEY_ID=$AWS_ACCESS_KEY_ID
ENV AWS_SECRET_ACCESS_KEY=$AWS_SECRET_ACCESS_KEY
ENV CACHEPOT_BUCKET=$CACHEPOT_BUCKET
ENV CHEPOT_S3_KEY_PREFIX=$CHEPOT_S3_KEY_PREFIX

WORKDIR /anagolay

COPY . .

RUN cargo build "--$PROFILE" 

# Final stage. Copy the node executable and the script
# FROM alpine:latest ## this didn't work with ldd
FROM debian:10-slim as node
LABEL maintainer="daniel@woss.io" 
LABEL description="2nd stage image. Here copy the binary. Production ready image."

ENV RUST_BACKTRACE 1

ARG PROFILE=release

RUN useradd -m -u 1000 -U -s /bin/sh -d /anagolay anagolay && \
  mkdir -p /anagolay/.local/share/anagolay && \
  chown -R anagolay:anagolay /anagolay/.local && \
  ln -s /anagolay/.local/share/anagolay /data

COPY --from=builder /anagolay/target/$PROFILE/anagolay /usr/local/bin
COPY --from=builder /anagolay/scripts/10-cleanup.sh 10-cleanup.sh

# Shrinking

# checks https://linoxide.com/ldd-command-examples-linux/
RUN ldd /usr/local/bin/anagolay && \
  /usr/local/bin/anagolay --version

RUN  bash 10-cleanup.sh && \
  rm -rf 10-cleanup.sh

USER anagolay

EXPOSE 30333 9933 9944

VOLUME ["/data"]

# https://phoenixnap.com/kb/docker-cmd-vs-entrypoint
# ENTRYPOINT  ["/usr/local/bin/anagolay"]
CMD ["/usr/local/bin/anagolay"]
