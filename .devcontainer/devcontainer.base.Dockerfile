# https://github.com/microsoft/vscode-dev-containers/blob/v0.159.0/containers/rust/.devcontainer/base.Dockerfile
FROM mcr.microsoft.com/vscode/devcontainers/rust:1

ENV RUST_BACKTRACE=1
# Ensure apt is in non-interactive to avoid prompts
ENV DEBIAN_FRONTEND=noninteractive
# The node image comes with a base non-root 'node' user which this Dockerfile
# gives sudo access. However, for Linux, this user's GID/UID must match your local
# user UID/GID to avoid permission issues with bind mounts. Update USER_UID / USER_GID
# if yours is not 1000. See https://aka.ms/vscode-remote/containers/non-root-user.
# ARG USER_UID=${UID:-1000}

# volume for caching the cargo packages
VOLUME /usr/local/cargo

# volume for the mounted app
VOLUME /app

# expose the
EXPOSE 30333 9933 9944 6080 5901 9222

COPY scripts /scripts

# add debian 11 packages so we can get new neovim and zsh
COPY assets/bullseye.list /etc/apt/sources.list.d/

# for now let's keep old zshrc
RUN cp /etc/zsh/zshrc /opt/zshrc_old

RUN /scripts/01-install-system-dependencies.sh 
RUN apt-get -y -o DPkg::Options::="--force-overwrite" -o DPkg::Options::="--force-confdef" install zsh fonts-powerline neovim tree
RUN /scripts/02-setup-toolchain.sh 

# default shell is zsh for vscode user
RUN chsh -s $(which zsh) vscode

# improve the perf for the caching the cargo install and build
# RUN cargo install sccache
# RUN ls -la /usr/local/cargo/bin/sccache
# RUN sccache -V
# set the env so the sccache can be used
# ENV RUSTC_WRAPPER=$CARGO_HOME/bin/sccache

RUN /scripts/03-install-cargo-packages.sh

# for the vscode
RUN /scripts/99-install-dev-cargo-packages.sh

# self-destruct
RUN rm -rf /scripts
RUN apt-get autoremove -y


# Install docker and docker-compose (for setup)
# RUN curl https://get.docker.com/builds/Linux/x86_64/docker-latest.tgz | tar xvz -C /tmp/ && mv /tmp/docker/docker /usr/bin/docker \
# && curl -L "https://github.com/docker/compose/releases/download/1.24.1/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose \
# && chmod 755 /usr/local/bin/docker-compose


# COPY assets/gpg-agent.conf $HOME/.gnupg/gpg-agent.conf
RUN mkdir -p /home/vscode/.cache/zsh4humans/v5
RUN curl -fsSL -- https://raw.githubusercontent.com/romkatv/zsh4humans/v5/z4h.zsh > /home/vscode/.cache/zsh4humans/v5/z4h.zsh

COPY assets/dotfiles/.zshrc /home/vscode/.zshrc
COPY assets/dotfiles/.p10k.zsh /home/vscode/.p10k.zsh
COPY assets/dotfiles/.zshenv /home/vscode/.zshenv

# Maybe we need to remove this but for now let's keep it as a ref
# RUN rm -rf /home/vscode/.oh-my-bash /home/vscode/.oh-my-zsh

# change all the files to belong to vscode
RUN chown -R vscode:vscode /home/vscode /usr/local/cargo /dev/pts/0

# check do we have stuff installed
RUN rustc --version
RUN cargo --version
RUN cachepot --version

