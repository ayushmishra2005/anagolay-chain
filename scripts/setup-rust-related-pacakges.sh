#!/usr/bin/env bash

echo "***** INSTALLING RUST RELATED PACKAGES *****"

set -x
set -o errexit

curl https://ipfs.anagolay.network/ipfs/bafybeifsdbwxkaidfw4hwjzeghkerqwr4jwruvcbyddarl44ucmrdgihky >/usr/local/bin/cargo-chef
curl https://ipfs.anagolay.network/ipfs/bafybeigocpf3bgi24ytoeosa4muxltoxsw3ftbzq2t7zvpwrw2k6r6bqt4 >/usr/local/bin/makers
curl https://ipfs.anagolay.network/ipfs/bafybeibsxtln53slskrmv6avwnjofxu5qfpmawezh7xzxuxhrhpe23nkym >/usr/local/bin/sccache
curl https://ipfs.anagolay.network/ipfs/bafybeihxmsqdck7os7pplccqmqe2mrilp5ftsb6s3jxa2qyoih3p5akboa >/usr/local/bin/wasm-pack

chmod +x /usr/local/bin/cargo-chef &&
  chmod +x /usr/local/bin/makers &&
  chmod +x /usr/local/bin/sccache &&
  chmod +x /usr/local/bin/wasm-pack &&
  makers --version &&
  cargo-chef --version &&
  sccache --version &&
  wasm-pack --version

if [ -z $CI_PROJECT_NAME ]; then
  rustup update nightly
  rustup update stable
fi

#### make this optional and with the flag fo the dev contianer
# rustup component add rls
# rustup component add llvm-tools-preview
# cargo install taplo-cli
#### make this optional and with the flag fo the dev contianer
