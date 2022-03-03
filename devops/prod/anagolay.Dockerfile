ARG GIT_LATEST_REVISION
ARG CI_LINUX_TAG="050979ef"
ARG OCI_REGISTRY="docker.io"
####### BUILDER image
FROM ${OCI_REGISTRY}/anagolay/ci-linux:$CI_LINUX_TAG as builder

ENV RUSTC_WRAPPER=sccache

WORKDIR /build

COPY . . 

# build anagolay runtime and copy the artifact to the folder for later copy
RUN makers --disable-check-for-updates --profile production build-release  \
	&& mkdir artifacts \ 
	&& cp target/release/anagolay artifacts


# Final stage. Copy the node executable and the script
FROM docker.io/bitnami/minideb:bullseye as runtime

LABEL description="Production ready image for Anagolay: a platform for web3 Rights Management" \
	network.anagolay.image.type="main" \
	network.anagolay.image.authors="daniel@woss.io, devops-team@anagolay.network" \
	maintainer="daniel@woss.io" \
	network.anagolay.image.vendor="Anagolay Foundation" \
	network.anagolay.image.description="Anagolay: a platform for web3 Rights Management" \
	network.anagolay.image.source="https://gitlab.com/anagolay/anagolay/blob/${GIT_LATEST_REVISION}/devops/prod/anagolay.Dockerfile" \
	network.anagolay.image.documentation="https://gitlab.com/anagolay/anagolay"

COPY --from=builder /build/artifacts/anagolay /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /anagolay anagolay \
	&& mkdir -p /anagolay/.local/share/anagolay \
	&& chown -R anagolay:anagolay /anagolay/.local \
	&& ln -s /anagolay/.local/share/anagolay /data \
	&& ldd /usr/local/bin/anagolay \
	&& apt-get autoremove -y \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* \
    && rm -rf /usr/lib/python* \
    && rm -rf /usr/bin /usr/sbin /usr/share/man \
	&& anagolay --version \
	&& ls -h /usr/local/bin/anagolay

USER anagolay

EXPOSE 30333 9933 9944

VOLUME ["/data"]

# https://phoenixnap.com/kb/docker-cmd-vs-entrypoint
ENTRYPOINT  ["/usr/local/bin/anagolay"]

