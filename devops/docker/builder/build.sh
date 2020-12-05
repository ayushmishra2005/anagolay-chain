#!/usr/bin/env bash

set -e
set xe
echo "---- Building the base box"

time docker build -t sensiogroup/network-node-builder:latest -f devops/docker/builder/Dockerfile .

echo "---- Pushing the base box"
time docker push sensiogroup/network-node-builder:latest

# docker image tag sensiogroup/network-node-builder  registry.gitlab.com/sensio_group/network-node/builder
