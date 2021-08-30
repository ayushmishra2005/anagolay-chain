# Anagolay Network Node

Anagolay is a next-generation framework for ownerships, copyrights and digital licenses. 🚀

## Local Development

The installation assumes you are running Debian 10. This will install all the needed deps.

```sh
# OPTIONAL -- install this only if you are missing the system dependencies
sudo ./scripts/01-install-system-dependencies.sh

# MANDATORY -- this sets up the toolchains and rust
./scripts/02-setup-toolchain.sh

# MANDATORY -- Installs the additional packages for better Developer Experience (DX)
./scripts/03-install-cargo-packages.sh

# OPTIONAL [HIGHLY RECOMMENDED]-- For better DX while developing the project + some goodies like cargo-cache
./scripts/99-install-dev-cargo-packages.sh

# Initialize the git hooks described in the rusty-hooks.toml
rusty-hook init
```

### Speeding up your build times with `cachepot` and local driver

Before continuing check that your `RUSTC_WRAPPER` is not set by running this command:

```bash
echo $RUSTC_WRAPPER
```

If the output is something than blank line run this:

```bash
unset RUSTC_WRAPPER
```

Now we can start installing it and setting it up. Follow the code snippet to get it installed:

```bash
# Install the cachepot
cargo install --git https://github.com/paritytech/cachepot
```

After that's done open your `.zshrc` or `.bashrc` and add following at the end of the file:

```bash
# this will pick up the location of the installation
export CACHEPOT_DIR="$HOME/.cache/cachepot"

# recommended location for the caches
export RUSTC_WRAPPER="$HOME/.cargo/bin/cachepot"

# reload your shell

## plain old sourcing
source ~/.zshrc
#or
source  ~/.bashrc

# if you are using the zsh4humans ignore sourcing above and execute this
exec zsh
```

To test that we have correctly installed the cachepot we need to compile the project.

```bash
cargo make check && cachepot -s
```

If everything is ok, you should see statistics about your cache.

### Testing, building and available commands

We are using the [cargo-make](https://github.com/sagiegurari/cargo-make) crate to automate scripts and make them easier to maintain and execute. All scripts are semantically named and `cargo` top level commands are the same.

Here is the list of currently available tasks which you can run by `cargo make TASK_NAME`:

| Task name                | description                                                                                      |
| :----------------------- | :----------------------------------------------------------------------------------------------- |
| fmt                      | Formats the code using the `cargo fmt`                                                           |
| fmt-check                | Checks the code using the `cargo fmt -- --check`                                                 |
| check                    | Check a anagolay node and all of its dependencies for errors.                                    |
| clean                    | Remove generated artifacts, namely `anagolay`.                                                   |
| build                    | Compile the Anagolay runtime in debug mode. Accepts any valid build arguments.                   |
| build-release            | Compile the Anagolay runtime in release mode. Accepts any valid build arguments.                 |
| build-release-benchmarks | Compile the Anagolay runtime in release mode with feature flag for benchmarks.                   |
| test                     | Execute unit and integration tests of a anagolay node. Accepts any valid `cargo test` arguments. |
| test-benchmarking        | Execute unit and integration tests of a anagolay node with flags enabled for testing benchmarks. |
| chain-dev-purge          | Purge the local chain database for debug build.                                                  |
| chain-dev-start          | Starts the chain in dev mode with sane default flags.                                            |
| ci-flow                  | Experimental ci-flow which runs the checks in single run instead many.                           |

### VsCode

For your convenience we provide the settings, recommended extensions and devcontainer. Feel free to use it and report any issues you face.

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

Build Code coverage:

```sh
cargo tarpaulin --output-dir coverage --out html
```

Trivy docker conatiner scaning

```sh
docker pull aquasec/trivy:0.16.0
docker run --rm -v /tmp/trivy/:/root/.cache/ aquasec/trivy:0.16.0  anagolay/node:0.2.1-dev
```

## Creating the docker images

Building the Rust-CI base image. This image is used as a CI image on gitlab and as a base builder image for the anagolay node. To change the version edit the `.env` file variable `RUST_IMAGE_VERSION`.

This is quite big image :

```bash
anagolay/rust-ci       0.1.0                9724ca9f4474   3 minutes ago    3.09GB
anagolay/rust-ci       latest               9724ca9f4474   3 minutes ago    3.09GB
```

TODO: pick up the correct version from [`runtime/Cargo.toml`](./runtime/Cargo.toml)

```bash
docker-compose -f rust-ci-docker-compose.yml build
docker-compose -f rust-ci-docker-compose.yml push
```

To build the Anagolay Node you should:

1.  have the latest base image
2.  change the correct version in `.env` file
3.  run the script

```bash
# build it
docker-compose build node

# push it
docker-compose push node
```

## Running in dev mode

```bash
cargo make chain-dev-start
```

## Testing

To test the full suite run `cargo make test`

## Benchmarking

To generate pallet weights run from the root of the project. Use the folder name for the pallet. It will add the `an_` prefix to it. This script will compile the node for you and run the benchmarks.

```sh
# TEMPLATE  ./scripts/run-benchmarks.sh CHAIN PALLET $RUN_WITH_BUILD[true << default |false]

# PoE
./scripts/run-benchmarks.sh dev poe false # this will not build it it will only run it

# Rules
./scripts/run-benchmarks.sh dev rules # this will build it and run it

# Operations
./scripts/run-benchmarks.sh dev operations


# All pallets and all extrinsics with the release build
./scripts/run-all-benchmarks.sh dev

# All pallets and all extrinsics without the release build
./scripts/run-all-benchmarks.sh dev false

```

## NOTES

### Failing cachepot

One of the fastest ways is to `unset RUSTC_WRAPPER` and run the commands again. If you are doing `cargo make build`, and you unset the RUSTC_WRAPPER then the command should finish normally, but it will take longer time.

https://github.com/scs/substrate-api-client/blob/master/test_no_std/Cargo.toml

`find . -name "\*.sh" -exec chmod +x {} \;`

`export COMPOSE_DOCKER_CLI_BUILD=0 && export DOCKER_BUILD=0 && export DOCKER_BUILDKIT=0`
`export COMPOSE_DOCKER_CLI_BUILD=1 && export DOCKER_BUILD=1 && export DOCKER_BUILDKIT=1`

Update the `*.sh` in scripts to be executable. Do this after you use the `chmod +x ./script.sh`

```
git ls-files -z scripts/*.sh | xargs -0 git update-index --chmod=+x
```
