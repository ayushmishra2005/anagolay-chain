#!/usr/bin/env bash
export HISTIGNORE='DOPPLER_*'

PROJECT_ROOT=$(git rev-parse --show-toplevel)

sccache --stop-server

ln -fs $GITPOD_REPO_ROOT/.devops/gitpod/.bash_aliases $HOME/.bash_aliases
bash $GITPOD_REPO_ROOT/.devops/gitpod/prep-doppler.sh

bash $GITPOD_REPO_ROOT/.devops/scripts/setup-rust-related-pacakges.sh dev
