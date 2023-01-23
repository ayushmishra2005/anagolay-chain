#!/usr/bin/env bash
export HISTIGNORE='DOPPLER_*'

# PROJECT_ROOT=$(git rev-parse --show-toplevel)

sccache --stop-server

# brew install starship romkatv/powerlevel10k/powerlevel10k
brew install romkatv/powerlevel10k/powerlevel10k

ln -fs "$GITPOD_REPO_ROOT"/.devops/gitpod/.bash_aliases "$HOME"/.bash_aliases
ln -fs "$GITPOD_REPO_ROOT"/.devops/gitpod/.zshrc "$HOME"/.zshrc
ln -fs "$GITPOD_REPO_ROOT"/.devops/gitpod/.p10k.zsh "$HOME"/.p10k.zsh

rm -rf ~/.tmux
git clone https://github.com/gpakosz/.tmux.git ~/.tmux
ln -sf ~/.tmux/.tmux.conf ~/.tmux.conf

if [ ! -f "$HOME/.tmux.conf.local" ]; then
  # wget https://ipfs.anagolay.network/ipfs/QmdZFrnc6NwzKSQdxkZfxHaBXMDH3ndhtwSm7dB7L1NXvM -O $HOME/.tmux.conf
  ln -fs "$GITPOD_REPO_ROOT"/.devops/gitpod/.tmux.conf.local "$HOME"/.tmux.conf.local
fi

bash $GITPOD_REPO_ROOT/.devops/gitpod/prep-doppler.sh
bash $GITPOD_REPO_ROOT/.devops/scripts/setup-rust-related-pacakges.sh dev

echo "Downloading anagolay ipfs CLI which works with the ipfsAuthProxy."
sudo sh -c "wget -q https://ipfs.anagolay.network/ipfs/bafybeig634knkl57gqgkmh3fti6zxisfcd47swetf5lastcx2waboa4a4a -O /usr/local/bin/ipfsCli"
sudo chmod +x /usr/local/bin/ipfsCli

exit
