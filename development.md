# For Developers among us

## Running in dev mode

```bash
makers start
```

## Start the release chain

```sh
mkdir -p resources
./target/production/anagolay --base-path ./target/data --chain local --no-telemetry --rpc-methods Unsafe --rpc-cors all --unsafe-rpc-external
./target/production/anagolay build-spec --base-path ./target/data --disable-default-bootnode --chain local > ./resources/idiyanale/spec.json
./target/production/anagolay build-spec --chain=./resources/idiyanale/spec.json  --raw --disable-default-bootnode > ./resources/idiyanale/genesis.json
```

## CI commit messages

The gitlab job [`test`](./.devops/ci/gitlab/jobs/test.yml) is configured to run with the [`.rules-run-if-source-code-is-changed`](./.devops/ci/gitlab/utils/rules.yml#20) rule which triggers it in one of two cases:

1. if the commit message starts with `[sc build]`
2. any changes in rust files are made in the `node`, `pallets`, `runtime` directories

## sccache

when you first time create the workspace you will need to do this:

```bash
# to get the envs
direnv allow

# stop the server which started without the env variables and it's using the local cache
sccache --stop-server

# to start the server and verify that it is using the aws as a storge
sccache --start-server; sccache --show-stats
```

## gitlab and jobs

### Extending the jobs

Links https://docs.gitlab.com/ee/ci/yaml/yaml_optimization.html#use-extends-to-reuse-configuration-sections, https://docs.gitlab.com/ee/ci/yaml/#extends , https://docs.gitlab.com/ee/ci/yaml/yaml_optimization.html#use-extends-and-include-together
With extend pay attention on the rules of merging.

```
SCCACHE_START_SERVER=1 SCCACHE_NO_DAEMON=1 RUST_LOG=sccache=trace SCCACHE_LOG=debug sccache

sccache --stop-server && SCCACHE_START_SERVER=1 SCCACHE_NO_DAEMON=1 RUST_LOG=sccache=trace SCCACHE_LOG=debug sccache
```

## Troubleshooting

If you happen to see the `profraw` files they are used to instrument your code. Here is what you can do:

1. check the RUSTFLAGS with `echo $RUSTFLAGS`, this should not contain the `-C instrument-coverage`
2. completely unset the RUSTFLAGS with `unset RUSTFLAGS`
3. remove all the `profraw` files using this `find . \( -name "*.profraw" \) -delete`

## Security and code quality

To create the Anagolay node you need to have latest version of the base image `anagolay/rust-ci` which you can get it from [DockerHub](https://hub.docker.com/r/anagolay/rust-ci)

Scanning for the vulnerabilities using builtin [Docker scan](https://docs.docker.com/engine/scan/):

```sh
docker scan --file Dockerfile --exclude-base anagolay/node:0.2.1-dev
```

Audit the code using the [Cargo Audit](https://github.com/RustSec/cargo-audit):

```sh
# check the security
cargo audit
```

## Code coverage

`grcov` produces the correct output in either HTML or GitLab compatible format.
Total process takes around 40 min on Gitpod - roughly 20 for instrumentation, 10 for tests, 10 for report generation:

**Instrumentation**

```sh
rustup component add llvm-tools-preview
cargo install grcov

export LLVM_PROFILE_FILE="anagolay-%p-%m.profraw"
export RUSTFLAGS="-Cinstrument-coverage"
export SKIP_WASM_BUILD=true

makers clean
makers build
makers test
```

**HTML report generation**

This will generate a static website in a folder (`./coverage`), including badges:

```sh
grcov . -s . -t html --binary-path ./target/debug --llvm --branch --ignore-not-existing --ignore "/*" -o ./coverage
```

**Cobertura report generation**

This will generate an XML file (`./coverage.xml`) suitable by GitLab pipelines:

```sh
grcov . -s . -t cobertura --binary-path ./target/debug --llvm --branch --ignore-not-existing --ignore "/*" -o ./coverage.xml
```

**Cleanup**

```sh
find . \( -name "anagolay*.profraw" \) -delete
```

## Testing

To test the full suite run `makers test`
To test documentation examples, run `makers test --doc -- --show-output`

## Benchmarking

To generate pallet weights run from the root of the project. Use the folder name for the pallet. This script will compile the node for you and run the benchmarks.

```sh
# TEMPLATE  ./.devops/run-benchmarks.sh CHAIN PALLET $RUN_WITH_BUILD[true << default |false]

# PoE
./.devops/run-benchmarks.sh dev poe false # this will not build it it will only run it

# Workflows
./.devops/run-benchmarks.sh dev workflows # this will build it and run it

# Operations
./.devops/run-benchmarks.sh dev operations


# All pallets and all extrinsics with the release build
./.devops/run-all-benchmarks.sh dev

# All pallets and all extrinsics without the release build
./.devops/run-all-benchmarks.sh dev false

```

## Zombienet

[zombienet](https://github.com/paritytech/zombienet) allows to quickly setup a test environment with a relay chain and two collators, running
the debug (or the release) build.
The network is started by `start` cargo target and requires to have the binaries of `polkadot` and `zombienet` in the path.
It will spawn native (non containerized) nodes and will be configured by the file `zombienet/config.toml`.
In order to have the required binaries you can run the following shell script on your local setup:

```sh
echo '#!/bin/sh
docker run -v /tmp:/tmp -u $(id -u):$(id -g) paritypr/polkadot-debug:master $@' > polkadot && sudo mv polkadot /usr/local/bin/polkadot
sudo chmod +x /usr/local/bin/polkadot

sudo sh -c "wget -c https://github.com/paritytech/zombienet/releases/download/v1.3.33/zombienet-linux-x64 -O /usr/local/bin/zombienet"
sudo chmod +x /usr/local/bin/zombienet
```

## Documentation

Cargo has integration with rustdoc to make it easier to generate docs. To generate pallets/crates documentation run from the root of the project. It'll generate all pallet's documentation.

```sh
makers docs-flow --no-deps
```

`docs/api` folder will contain the autogenerated docs in the root of the project.

VERY important website to check when upgrading the toolchain https://rust-lang.github.io/rustup-components-history/
