#!/usr/bin/env bash

set -e
set -x

echo "*** starting chain"

./target/debug/anagolay --dev -l error,runtime,info --no-telemetry --rpc-external --unsafe-ws-external
