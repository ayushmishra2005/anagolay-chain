#https://github.com/microsoft/vscode-dev-containers/blob/main/containers/rust/.devcontainer/base.Dockerfile
ARG VARIANT="bullseye"
FROM mcr.microsoft.com/vscode/devcontainers/rust:1-${VARIANT}

WORKDIR /workspace

ARG USER_UID=${UID:-1000}
ARG RUST_TOOLCHAIN="stable-2022-01-20"

ENV RUST_BACKTRACE=1 \
	DEBIAN_FRONTEND=noninteractive \
	CARGO_HOME=/usr/local/cargo \
	RUSTUP_HOME=/usr/local/rustup \
	RUSTUP_PROFILE=default \
	PATH=${CARGO_HOME}/bin:${PATH} 

# volume for the mounted app
VOLUME /workspace

ENV RUSTC_WRAPPER=sccache

ENV SCCACHE_DIR=${HOME}/.cache/sccache

# expose the
# EXPOSE 30333 9933 9944 6080 5901 9222

COPY scripts /tmp/library-scripts/
COPY rust-toolchain.toml .

RUN bash /tmp/library-scripts/install-deps.sh \
	&& bash /tmp/library-scripts/setup-rust-related-pacakges.sh \
	&& bash /tmp/library-scripts/install-common-crates.sh

USER vscode 

RUN rm -rf ~/.oh-my-zsh
# run the installation script  
RUN sh -c "$(wget https://raw.githubusercontent.com/robbyrussell/oh-my-zsh/master/tools/install.sh -O -)"

# install powerlevel10k
RUN git clone https://github.com/romkatv/powerlevel10k.git ~/.oh-my-zsh/custom/themes/powerlevel10k
# Syntax Plugin
RUN git clone https://github.com/zsh-users/zsh-syntax-highlighting.git ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-syntax-highlighting
# Autocomplete
RUN git clone https://github.com/zsh-users/zsh-autosuggestions ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-autosuggestions

USER root

RUN chown -R vscode:vscode /usr/local/cargo
RUN chown -R vscode:vscode /usr/local/rustup
RUN chown -R root:rustlang /usr/local/bin/*