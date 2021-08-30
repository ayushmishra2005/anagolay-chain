#!/usr/bin/env bash

set -e
set -x

echo "*** Upgrade system  ***"

apt-get update &&
    apt-get dist-upgrade -y -o Dpkg::Options::="--force-confold"
