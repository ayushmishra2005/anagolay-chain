#!/usr/bin/env bash

echo "copy local configs"

# Copy ssh creed from local home
# Needed for e.g. git push etc
mkdir -p ~/.ssh
cp -r /tmp/.ssh/* ~/.ssh
chmod 600 ~/.ssh/*




cp /app/assets/dotfiles/.env.zsh $HOME/.env.zsh