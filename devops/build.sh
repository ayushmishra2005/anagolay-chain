#!/usr/bin/env bash
set -x
# -a part will allow the sourcing of the env
set -a
set -o errexit

echo "THIS SHOULD NOT BE USED IN PRODUCTION!!!"
echo "IT IS ONLY FOR LOCAL BUILDING OF THE PRODUCTION IMAGE"
echo "IT WILL BE REMOVED AT SOME POINT"

# we might have idiyanale too
PROJECT_ROOT=$(git rev-parse --show-toplevel)
cd $PROJECT_ROOT

PROJECT=${1:-"anagolay"}
GIT_LATEST_REVISION=${2:-$(git rev-parse --short HEAD)}
FULL_IMAGE_NAME="anagolay/$PROJECT:$GIT_LATEST_REVISION"

OCI_REGISTRY="docker.io"

# source $PROJECT_ROOT/.env

echo "Building the base image ..."
docker build \
	--build-arg GIT_LATEST_REVISION=$GIT_LATEST_REVISION \
	--tag $OCI_REGISTRY/anagolay/ci-linux:$GIT_LATEST_REVISION \
	--file $PROJECT_ROOT/devops/prod/ci-linux.Dockerfile .

echo "Building the main image ..."
docker build \
	--build-arg GIT_LATEST_REVISION=$GIT_LATEST_REVISION \
	--tag $OCI_REGISTRY/$FULL_IMAGE_NAME \
	--file $PROJECT_ROOT/devops/prod/$PROJECT.Dockerfile .

# docker run --rm -it anagolay/anagolay:$GIT_LATEST_REVISION --dev --no-telemetry --rpc-external --unsafe-ws-external

# docker run --rm -it anagolay/anagolay:e42e183 --dev --no-telemetry --rpc-external --unsafe-ws-external
