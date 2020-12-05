#!/usr/bin/env bash

set -e
set -x

cargo make --makefile flow.toml dev-start
