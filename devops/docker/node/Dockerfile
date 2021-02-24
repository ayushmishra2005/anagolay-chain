FROM anagolay/network-node-builder:latest as builder
LABEL maintainer="daniel@woss.io"
LABEL description="This is the build box."

FROM phusion/baseimage:0.11
LABEL maintainer="daniel@woss.io"
LABEL description="This is the 2nd stage: a very small image where we copy the anagolay binary."

ARG PROFILE=release

# RUN apk add --no-cache ca-certificates \
#     libstdc++ \
#     openssl

RUN mv /usr/share/ca* /tmp && \
    rm -rf /usr/share/*  && \
    mv /tmp/ca-certificates /usr/share/ && \
    useradd -m -u 1000 -U -s /bin/sh -d /anagolay anagolay && \
    mkdir -p /anagolay/.local/share/anagolay && \
    chown -R anagolay:anagolay /anagolay/.local && \
    ln -s /anagolay/.local/share/anagolay /data

COPY --from=builder /anagolay/target/$PROFILE/anagolay /usr/local/bin
# 
RUN rm -rf /usr/lib/python* && \
    mkdir -p /root/.local/share/anagolay && \
    ln -s /root/.local/share/anagolay /data

# checks
RUN ldd /usr/local/bin/anagolay && \
    /usr/local/bin/anagolay --version

# Shrinking
RUN rm -rf /usr/lib/python* && \
    rm -rf /usr/bin /usr/sbin /usr/share/man


# let's see will this run in this container
RUN /usr/local/bin/anagolay --version

USER anagolay

EXPOSE 30333 9933 9944

VOLUME ["/data"]

CMD ["/usr/local/bin/anagolay"]
