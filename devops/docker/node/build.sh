#!/usr/bin/env bash

set -e

echo "---- Building the production box"
# time docker build -t sensiogroup/network-node-build:0.2.0-dev -f devops/docker/node/build.Dockerfile .

time docker build -t sensiogroup/network-node:0.2.0-dev -f devops/docker/node/Dockerfile .

echo "---- Pushing the production box"
# docker login registry.gitlab.com

# time docker push sensiogroup/network-node-build:0.2.0-dev
time docker push sensiogroup/network-node:0.2.0-dev
