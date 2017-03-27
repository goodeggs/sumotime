#!/bin/bash
set -e
set -o pipefail
#set -x

version=$(grep '^version\b' Cargo.toml | head -n1 | awk -F'"' '{print $2}')
releasedir="${PWD}/releases/${version}"

rm -rf "$releasedir" && mkdir -p "$releasedir"

echo "building for Darwin-x86_64"
cargo build --release --target=x86_64-apple-darwin
( cd target/x86_64-apple-darwin/release && cp sumotime "${releasedir}/sumotime-Darwin-x86_64" )

echo "building for Linux-x86_64"
cargo build --release --target=x86_64-unknown-linux-musl
( cd target/x86_64-unknown-linux-musl/release && cp sumotime "${releasedir}/sumotime-Linux-x86_64" )

echo "releasing v${version}..."
ghr -t "$GITHUB_TOKEN" -u goodeggs -r sumotime --replace "v${version}" "releases/${version}/"

