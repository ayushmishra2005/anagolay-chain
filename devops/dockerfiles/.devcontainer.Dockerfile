# https://github.com/microsoft/vscode-dev-containers/blob/v0.159.0/containers/rust/.devcontainer/base.Dockerfile
FROM mcr.microsoft.com/vscode/devcontainers/rust

ARG USER_UID=${UID:-1000}


ENV RUST_BACKTRACE=1 \
	DEBIAN_FRONTEND=noninteractive \
	CARGO_HOME=/usr/local/cargo \
	RUSTUP_HOME=/usr/local/rustup \
	RUSTUP_PROFILE=default \
	PATH=${CARGO_HOME}/bin:${PATH}


# volume for caching the cargo packages
VOLUME /usr/local/cargo

# volume for the mounted app
VOLUME /app

# expose the
# EXPOSE 30333 9933 9944 6080 5901 9222

COPY scripts/install-deps.sh /tmp/library-scripts/

RUN bash /tmp/library-scripts/install-deps.sh
