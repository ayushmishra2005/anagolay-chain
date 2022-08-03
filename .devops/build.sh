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

CI_COMMIT_SHORT_SHA=${2:-$(git rev-parse --short HEAD)}
GITLAB_GROUP="anagolay"
GITLAB_PROJECT="anagolay" # for runtime
FULL_IMAGE_NAME=$GITLAB_GROUP/$GITLAB_PROJECT:$CI_COMMIT_SHORT_SHA

# source $PROJECT_ROOT/.env

# echo "Building the ci-image ..."
# DOCKER_BUILDKIT=0 CI_COMMIT_SHORT_SHA=$CI_COMMIT_SHORT_SHA doppler run -- \
#   docker-compose \
#   -f .devops/docker/compose/ci-linux.yml \
#   build ci-linux

echo "Building the ci-image ..."
DOCKER_BUILDKIT=0 doppler run -- \
  docker-compose \
  -f .devops/docker/compose/anagolay.yml \
  build --no-cache \
  anagolay

# docker build \
#   --build-arg COMMAND_TO_EXEC="makers test" \
#   --tag anagolay/ci-jobs-helper-image \
#   --file $PROJECT_ROOT/.devops/docker/ci-jobs.dockerfile .

# docker build \
#   --build-arg GIT_LATEST_REVISION=$GIT_LATEST_REVISION \
#   --tag $OCI_REGISTRY/$GITLAB_GROUP/$GITLAB_PROJECT/ci-linux:$GIT_LATEST_REVISION \
#   --file $PROJECT_ROOT/.devops/prod/ci-linux.dockerfile .

# echo "Building the main image ..."
# docker build \
#   --build-arg GIT_LATEST_REVISION=$GIT_LATEST_REVISION \
#   --tag $OCI_REGISTRY/$FULL_IMAGE_NAME \
#   --file $PROJECT_ROOT/.devops/prod/$PROJECT.dockerfile .

# docker run --rm -it anagolay/anagolay:$GIT_LATEST_REVISION --dev --no-telemetry --rpc-external --unsafe-ws-external

# docker run --rm -it anagolay/anagolay:e42e183 --dev --no-telemetry --rpc-external --unsafe-ws-external
