# Anagolay Network Node

Anagolay is a next-generation framework for ownerships, copyrights and digital licenses. 🚀

## Local Development

Here is the list of currently available tasks `makers --list-category-steps anagolay` which you can run by `makers TASK_NAME`.

```txt
❯ makers --list-category-steps anagolay
[cargo-make] INFO - makers 0.35.15
[cargo-make] INFO - Build File: Makefile.toml
[cargo-make] INFO - Task: default
[cargo-make] INFO - Profile: development
anagolay
----------
build - Compile the Anagolay runtime in debug mode. Accepts any valid build arguments.
build-production - Compile the Anagolay runtime in release mode with  option and custom profile `production`. Accepts any valid build arguments.
build-release-benchmarks - Compile the Anagolay runtime in release mode with feature flag for benchmarks.
start - Starts the chain in dev mode with sane default flags.
ci-flow-light - Experimental ci-flow which runs the checks in single run instead many.
clean-all - Remove generated artifacts.
clean-anagolay - Remove generated artifact `anagolay`.
test-benchmarking - Execute unit and integration tests of a anagolay node with flags enabled for testing benchmarks.
```

### Building

```sh
makers build
```

### Testing and available commands

We are using the [makers](https://github.com/sagiegurari/cargo-make) crate to automate scripts and make them easier to maintain and execute. All scripts are semantically named and `makers` top level commands are the same.

## Development

If you are interested in how to work with Anagolay node checkout `development.md` file

## License

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
