#!/usr/bin/env bash

set -e
set -x

echo "*** Cleaning up the  system ***"

apt-get autoremove -y &&
    apt-get clean &&
    rm -rf /var/lib/apt/lists/* &&
    rm -rf /usr/lib/python* &&
    rm -rf /usr/bin /usr/sbin /usr/share/man
