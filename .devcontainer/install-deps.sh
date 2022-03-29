#!/usr/bin/env bash

set -o errexit

cp .devcontainer/.zshrc $HOME
cp .devcontainer/.p10k.zsh $HOME

cargo install miniserve
