#!/usr/bin/env bash

PROJECT_ROOT=$(git rev-parse --show-toplevel)

echo "Update and install system deps:"

sudo bash ./scripts/install-deps.sh
bash ./scripts/install-common-crates.sh
sudo bash ./scripts/setup-rust-related-pacakges.sh

echo "Installing the Anagolay IPFS upload"
sudo sh -c 'curl https://ipfs.anagolay.network/ipfs/bafybeifnjzcbu76ivm22w3x37pnqqnha753ity5divosgvdnfu5ybsquji > /usr/local/bin/ipfsCli && chmod +x /usr/local/bin/ipfsCli'

echo "Installing the Remote Signer"
sudo sh -c 'curl https://ipfs.anagolay.network/ipfs/bafybeiarhwobvpvz76iy6clqaf3ub7yc4rvvkydmimh652r2svdaznubrq > /usr/local/bin/remote-signer && chmod +x /usr/local/bin/remote-signer'
git config --global gpg.program "remote-signer"
git config --global commit.gpgsign true

echo "Installing rustup"
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
rustc -V

echo "Download and install binaryen"
wget https://github.com/WebAssembly/binaryen/releases/download/version_90/binaryen-version_90-x86-linux.tar.gz

tar xvf binaryen-version_90-x86-linux.tar.gz
mv $PROJECT_ROOT/binaryen-version_90/* $CARGO_HOME/bin/
rm -f binaryen-version_90-x86-linux.tar.gz

echo "Installing homegrew packages"
brew install fzf

# install pnpm
echo "Installing pnpm"
npm install -g pnpm

echo "Smoke test"
rustup --version &&
  rustc --version &&
  cargo --version &&
  git --version &&
  tar --version &&
  pnpm --version &&
  node --version &&
  makers --version &&
  wasm-pack --version &&
  wasm-bindgen --version &&
  ipfsCli --version

# if [ ! -d "$HOME/.oh-my-zsh" ]; then
#   echo "Setting up zsh and the terminal theme"
#   sh -c "$(wget https://raw.githubusercontent.com/robbyrussell/oh-my-zsh/master/tools/install.sh -O -)"

#   if [ ! -d "$HOME/.oh-my-zsh/custom/themes/powerlevel10k" ]; then
#     git clone https://github.com/romkatv/powerlevel10k.git $HOME/.oh-my-zsh/custom/themes/powerlevel10k
#   fi
#   if [ ! -d "$HOME/.oh-my-zsh/custom/plugins/zsh-syntax-highlighting" ]; then
#     git clone https://github.com/zsh-users/zsh-syntax-highlighting.git $HOME/.oh-my-zsh/custom/plugins/zsh-syntax-highlighting
#   fi
#   if [ ! -d "$HOME/.oh-my-zsh/custom/plugins/zsh-autosuggestions" ]; then
#     git clone https://github.com/zsh-users/zsh-autosuggestions $HOME/.oh-my-zsh/custom/plugins/zsh-autosuggestions
#   fi
# fi
