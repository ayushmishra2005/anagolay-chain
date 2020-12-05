FROM sensiogroup/network-node-builder:latest as builder
LABEL maintainer="daniel@woss.io"
LABEL description="This is the build box."

FROM phusion/baseimage:0.11
LABEL maintainer="daniel@woss.io"
LABEL description="This is the 2nd stage: a very small image where we copy the sensio binary."

ARG PROFILE=release

# RUN apk add --no-cache ca-certificates \
#     libstdc++ \
#     openssl

RUN mv /usr/share/ca* /tmp && \
    rm -rf /usr/share/*  && \
    mv /tmp/ca-certificates /usr/share/ && \
    useradd -m -u 1000 -U -s /bin/sh -d /sensio sensio && \
    mkdir -p /sensio/.local/share/sensio && \
    chown -R sensio:sensio /sensio/.local && \
    ln -s /sensio/.local/share/sensio /data

COPY --from=builder /sensio/target/$PROFILE/sensio /usr/local/bin
# 
RUN rm -rf /usr/lib/python* && \
    mkdir -p /root/.local/share/sensio && \
    ln -s /root/.local/share/sensio /data

# checks
RUN ldd /usr/local/bin/sensio && \
    /usr/local/bin/sensio --version

# Shrinking
RUN rm -rf /usr/lib/python* && \
    rm -rf /usr/bin /usr/sbin /usr/share/man


# let's see will this run in this container
RUN /usr/local/bin/sensio --version

USER sensio

EXPOSE 30333 9933 9944

VOLUME ["/data"]

CMD ["/usr/local/bin/sensio"]
