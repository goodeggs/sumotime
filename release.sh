#!/bin/bash
set -e
set -o pipefail
#set -x

version=$(cat VERSION)
releasedir="${PWD}/releases/${version}"

rm -rf "$releasedir" && mkdir -p "$releasedir"

echo "building for darwin_amd64"
cargo build --release --target=x86_64-apple-darwin
cd target/x86_64-apple-darwin/release && zip ${releasedir}/sumotime_v${version}_darwin_amd64.zip sumotime
cd -

echo "building for linux_amd64"
cargo build --release --target=x86_64-unknown-linux-musl
cd target/x86_64-unknown-linux-musl/release && tar czvf ${releasedir}/sumotime_v${version}_linux_amd64.tar.gz sumotime
cd -

echo "releasing v${version}..."
ghr -t "$GITHUB_TOKEN" -u goodeggs -r sumotime --replace "v${version}" "releases/${version}/"

