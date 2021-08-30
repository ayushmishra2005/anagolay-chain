#!/usr/bin/env bash

SINGLE_DATES=$(grep -lr "// Copyright (C) [0-9]* Anagolay Foundation.")
YEAR=$(date +%Y)

for file in $SINGLE_DATES; do
  FILE_YEAR=$(cat $file | sed -n "s|// Copyright (C) \([[:digit:]][[:digit:]][[:digit:]][[:digit:]]\) Anagolay Foundation.|\1|p")
  if [ $YEAR -ne $FILE_YEAR ]; then
    sed -i -e "s|// Copyright (C) \([[:digit:]][[:digit:]][[:digit:]][[:digit:]]\) Anagolay Foundation.|// Copyright (C) \1-$YEAR Anagolay Foundation.|g" $file
  fi
done

grep -lr "// Copyright (C) [0-9]*-[0-9]* Anagolay Foundation." |
  xargs sed -i -e "s|// Copyright (C) \([[:digit:]][[:digit:]][[:digit:]][[:digit:]]\)-[[:digit:]][[:digit:]][[:digit:]][[:digit:]] Anagolay Foundation.|// Copyright (C) \1-$YEAR Anagolay Foundation.|g"
