FROM gitpod/workspace-full

# Install custom tools, runtime, etc.
RUN brew install fzf

COPY scripts/install-deps.sh /tmp/

RUN bash /tmp/install-deps.sh