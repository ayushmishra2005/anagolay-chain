#!/usr/bin/env bash

# -a part will allow the sourcing of the env
set -a

# Find the doc root
ROOT_DIR=$(git rev-parse --show-toplevel)

source $ROOT_DIR/.env

export COMPOSE_DOCKER_CLI_BUILD=1 && export DOCKER_BUILD=1 && export DOCKER_BUILDKIT=1

docker build \
  --tag anagolay/node:$ANAGOLAY_VERSION \
  --build-arg AWS_ACCESS_KEY_ID \
  --build-arg AWS_SECRET_ACCESS_KEY \
  --build-arg CACHEPOT_BUCKET \
  --build-arg ACHEPOT_S3_KEY_PREFIX \
  -f Dockerfile \
  --no-cache \
  .
