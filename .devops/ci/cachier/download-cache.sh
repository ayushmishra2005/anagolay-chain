#!/usr/bin/env bash

echo -e "\e[0Ksection_start:$(date +%s):download-section\r\e[0KDownload || Extract cache"

if [ -f "$BUILD_CACHE_FILE" ]; then
  echo "➡️ Unzipping the Build cache..."
  unzip -q -o "$BUILD_CACHE_FILE"
# else
#   echo "➡️ Copying the prebuilt image cache... REMOVE THIS FROM HERE FIRST THEN REBUILD THE CI-IMAGE"
#   cp -R "$PREBUILT_CACHE_TARGET_DIR" "$CI_PROJECT_DIR"
#   # echo "  Linking the prebuilt image cache..."
#   # ln -sf $PREBUILT_CACHE_TARGET_DIR $CI_PROJECT_DIR/target
fi

if [ -f "$SCCACHE_CACHE_FILE" ]; then
  echo "➡️ Unzipping the sccache cache..."
  unzip -q -o "$SCCACHE_CACHE_FILE"
  du -sh .sccache
  echo "current path $(pwd)"
fi

if [ -f "$CARGO_CACHE_FILE" ]; then
  echo "➡️ Unzipping the cargo cache..."
  unzip -q -o "$CARGO_CACHE_FILE"
fi

echo -e "\e[0Ksection_end:$(date +%s):download-section\r\e[0K"
