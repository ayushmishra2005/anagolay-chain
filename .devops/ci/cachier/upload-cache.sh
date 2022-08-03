#!/usr/bin/env bash

echo -e "\e[0Ksection_start:$(date +%s):section\r\e[0KUpload caches"

# in case we don't have it
mkdir -p "$CACHE_DIR"

if [ -d "$CI_PROJECT_DIR"/target ]; then
  echo "➡️ target size $(du -sh "$CI_PROJECT_DIR"/target)"
  echo "➡️ zipping the target"
  time zip -q -0 -r - target/ >"$BUILD_CACHE_FILE"
fi

if [ -d "$CI_PROJECT_DIR"/.sccache ]; then
  echo "➡️ .sccache size $(du -sh "$CI_PROJECT_DIR"/.sccache)"
  echo "➡️ zipping the .sccache"
  time zip -q -0 -r - .sccache/ >"$SCCACHE_CACHE_FILE"
fi

if [ -d "$CI_PROJECT_DIR"/.cargo ]; then
  echo "➡️ .cargo size $(du -sh "$CI_PROJECT_DIR"/.cargo)"
  echo "➡️ zipping the .cargo"
  time zip -q -0 -r - .cargo/ >"$CARGO_CACHE_FILE"
fi

##### don't forget to clean up the cache on merge to default branch
echo -e "\e[0Ksection_end:$(date +%s):section\r\e[0K"
