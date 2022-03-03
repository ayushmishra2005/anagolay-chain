FROM docker.io/rust:1-bullseye as prebase

ARG GIT_LATEST_REVISION
ENV RUST_BACKTRACE=1 \
	# SCCACHE_DIR="/root/.cache/sccache" \	
	# RUSTC_WRAPPER="sccache" \
	CARGO_INCREMENTAL=1

WORKDIR /build

COPY scripts /tmp/library-scripts/
COPY rust-toolchain.toml .

RUN bash /tmp/library-scripts/install-deps.sh \
&& bash /tmp/library-scripts/setup-rust-related-pacakges.sh \
&& bash /tmp/library-scripts/install-common-crates.sh

# planing is good
FROM prebase as planner
WORKDIR /build
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# compiling is better
FROM prebase as cacher
WORKDIR /build
COPY --from=planner /build/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# gather all the caches to single image
FROM prebase
LABEL description="Builder image for Anagolay: a platform for web3 Rights Management" \
	network.anagolay.image.type="builder" \
	network.anagolay.image.authors="daniel@woss.io, devops-team@anagolay.network" \
	maintainer="daniel@woss.io" \
	network.anagolay.image.vendor="Anagolay Foundation" \
	network.anagolay.image.description="Anagolay: a platform for web3 Rights Management" \
	network.anagolay.image.source="https://gitlab.com/anagolay/anagolay/blob/${GIT_LATEST_REVISION}/devops/prod/base-ci.Dockerfile" \
	network.anagolay.image.documentation="https://gitlab.com/anagolay/anagolay"

WORKDIR /build

# Copy over the cached dependencies
COPY --from=cacher /build/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
COPY --from=cacher $SCCACHE_DIR $SCCACHE_DIR
