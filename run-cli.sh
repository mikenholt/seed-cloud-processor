#!/usr/bin/env bash
set -e

# Ensure script directory is CWD
pushd "${0%/*}" > /dev/null

UNAME=$(uname -s)

cargo run --

popd >/dev/null