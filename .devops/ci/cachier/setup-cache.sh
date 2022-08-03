#!/usr/bin/env bash

echo -e "\e[0Ksection_start:$(date +%s):setup-section\r\e[0KSetup caches"

# currently accepting only the `show-info`
ACTION=${1:-''}

if [ "$ACTION" = 'delete-project-cache' ]; then
  echo "Removing all the caches for $REPO_CACHE_DIR"
  rm -rf "$REPO_CACHE_DIR"
fi

## make the directories if the don't exist, if they do nothing will happen because of -p
echo "Creating $CARGO_HOME"
mkdir -p "$CARGO_HOME"

echo "Creating $SCCACHE_DIR"
mkdir -p "$SCCACHE_DIR"

echo "Creating $CACHE_DIR"
mkdir -p "$CACHE_DIR"

if [ "$ACTION" = 'show-info' ]; then
  if [ -d "$CACHE_DIR" ]; then
    echo "Cache size: $(du -sh "$CACHE_DIR")"
  fi

  # if [ -d "$PREBUILT_CACHE_TARGET_DIR" ]; then
  #   echo "Prebuilt Cache size: $(du -sh "$PREBUILT_CACHE_TARGET_DIR")"
  # fi

  if [ -f "$BUILD_CACHE_FILE" ]; then
    echo "Cache size: $(du -sh "$BUILD_CACHE_FILE")"
  fi
  if [ -f "$SCCACHE_CACHE_FILE" ]; then
    echo "Cache size: $(du -sh "$SCCACHE_CACHE_FILE")"
  fi

  if [ -f "$CARGO_CACHE_FILE" ]; then
    echo "Cache size: $(du -sh "$CARGO_CACHE_FILE")"
  fi
fi

echo "CI_COMMIT_REF_NAME is $CI_COMMIT_REF_NAME"
echo "CI_COMMIT_BRANCH is $CI_COMMIT_BRANCH"
echo "CI_DEFAULT_BRANCH is $CI_DEFAULT_BRANCH"
echo "CI_PIPELINE_SOURCE is $CI_PIPELINE_SOURCE"

echo -e "\e[0Ksection_end:$(date +%s):setup-section\r\e[0K"
