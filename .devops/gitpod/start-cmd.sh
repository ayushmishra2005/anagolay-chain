#!/usr/bin/env bash
export HISTIGNORE='DOPPLER_*'

PROJECT_ROOT=$(git rev-parse --show-toplevel)

sccache --stop-server

ln -fs $GITPOD_REPO_ROOT/.devops/gitpod/.bash_aliases $HOME/.bash_aliases
bash $GITPOD_REPO_ROOT/.devops/gitpod/prep-doppler.sh

bash $GITPOD_REPO_ROOT/.devops/scripts/setup-rust-related-pacakges.sh dev

echo "Downloading anagolay ipfs CLI which works with the ipfsAuthProxy."
sudo sh -c "wget -q https://ipfs.anagolay.network/ipfs/bafybeig634knkl57gqgkmh3fti6zxisfcd47swetf5lastcx2waboa4a4a -O /usr/local/bin/ipfsCli"
sudo chmod +x /usr/local/bin/ipfsCli

exit
