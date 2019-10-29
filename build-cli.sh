#!/usr/bin/env bash

set -e

# Ensure script directory is CWD
pushd "${0%/*}" > /dev/null

# VERSION=$1
# if [[ "${VERSION}x" == "x" ]]
# then
#     echo Missing version parameter - setting to snapshot
#     VERSION=snapshot
# fi

echo Building cross platform seed-batch CLI.
echo Building for Linux...
# cargo build --release --target x86_64-unknown-linux-musl
echo Building for OSX...
cargo build --release --target x86_64-apple-darwin
echo Building for Windows...
# cargo build --release --target x86_64-pc-windows-gnu
echo CLI build complete

popd >/dev/null

# if [[ "$2" == "--install-macos" ]]
# then
#     cp -f output/seed-batch-darwin-amd64 /usr/local/bin/seed-batch
#     echo Installed seed-batch to /usr/local/bin/seed-batch
# elif [[ "$2" == "--install-linux" ]]
# then
#     cp -f output/seed-batch-linux-amd64 /usr/local/bin/seed-batch
#     echo Installed seed-batch to /usr/local/bin/seed-batch
# fi